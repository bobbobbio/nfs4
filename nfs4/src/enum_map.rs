// Copyright 2023 Remi Bernotavicius

use derive_more::From;
use serde::{
    de::{DeserializeOwned, Deserializer},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read as _;

pub trait ToId<Id>
where
    Id: Sized,
{
    fn to_id(&self) -> Id;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EnumSet<K>(BTreeSet<K>);

impl<K> Default for EnumSet<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K> IntoIterator for EnumSet<K> {
    type Item = K;
    type IntoIter = std::collections::btree_set::IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K> FromIterator<K> for EnumSet<K>
where
    K: Ord,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = K>,
    {
        Self(BTreeSet::from_iter(iter))
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EnumMap<K, V>(BTreeMap<K, V>);

impl<K, V> Default for EnumMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K, V> FromIterator<V> for EnumMap<K, V>
where
    K: Ord,
    V: ToId<K>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = V>,
    {
        Self(BTreeMap::from_iter(
            iter.into_iter().map(|v| (v.to_id(), v)),
        ))
    }
}

impl<K> EnumSet<K>
where
    K: Into<u32> + Copy,
{
    fn to_raw(&self) -> EnumSetRaw {
        let mut map = vec![];
        for k in &self.0 {
            let k_as_num: u32 = (*k).into();

            let index = (k_as_num / 32) as usize;
            if index >= map.len() {
                map.resize(index + 1, 0);
            }
            let chunk = &mut map[index];
            *chunk |= 1 << (k_as_num % 32);
        }
        EnumSetRaw { map }
    }
}

impl<K, V> EnumMap<K, V>
where
    K: Ord + Copy,
{
    pub fn get(&self, key: K) -> Option<&V> {
        self.0.get(&key)
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.0.remove(&key)
    }
}

impl<K> EnumSet<K>
where
    K: Ord + Copy,
{
    pub fn remove(&mut self, key: K) -> bool {
        self.0.remove(&key)
    }
}

impl<K, V> EnumMap<K, V>
where
    K: Into<u32> + Copy + Ord,
    V: Serialize,
{
    fn to_raw(&self) -> EnumMapRaw {
        let keys: EnumSet<K> = self.0.keys().cloned().collect();
        let map = keys.to_raw();

        let mut body = vec![];
        for v in self.0.values() {
            let mut serialized = vec![];
            serde_xdr::to_writer(&mut serialized, v).unwrap();
            body.extend(serialized.into_iter().skip(4));
        }
        EnumMapRaw { map, body }
    }
}

impl<K, V> EnumMap<K, V>
where
    K: Ord,
{
    pub fn get_as<'a, T>(&'a self, key: K) -> Option<&'a T>
    where
        &'a T: TryFrom<&'a V>,
    {
        self.0.get(&key).map(|v| v.try_into().ok()).flatten()
    }

    pub fn remove_as<T>(&mut self, key: K) -> Option<T>
    where
        T: TryFrom<V>,
    {
        self.0.remove(&key).map(|v| v.try_into().ok()).flatten()
    }
}

#[derive(Debug, From)]
pub enum EnumMapDeserializationError {
    UnknownKeyValue(u32),
    BadValueXdr(serde_xdr::CompatDeserializationError),
}

impl<K> EnumSet<K>
where
    K: TryFrom<u32> + Ord + Serialize,
{
    fn try_from_raw(raw: EnumSetRaw) -> Result<Self, EnumMapDeserializationError> {
        let mut set = BTreeSet::new();
        for b in 0..(u32::try_from(raw.map.len()).unwrap() * 32) {
            let index = (b / 32) as usize;
            let chunk = &raw.map[index];
            if chunk & 1 << (b % 32) != 0 {
                let key: K = b
                    .try_into()
                    .map_err(|_| EnumMapDeserializationError::UnknownKeyValue(b))?;
                set.insert(key);
            }
        }
        Ok(Self(set))
    }
}

impl<K, V> EnumMap<K, V>
where
    K: TryFrom<u32> + Ord + Serialize,
    V: DeserializeOwned,
{
    fn try_from_raw(raw: EnumMapRaw) -> Result<Self, EnumMapDeserializationError> {
        let mut body_cursor = &raw.body[..];
        let mut map = BTreeMap::new();

        let key_set = EnumSet::try_from_raw(raw.map)?;

        for key in key_set {
            let serialized_key = serde_xdr::to_bytes(&key).unwrap();
            let mut combined_input = (&serialized_key[..]).chain(&mut body_cursor);
            let value: V = serde_xdr::from_reader(&mut combined_input)?;
            map.insert(key, value);
        }

        Ok(Self(map))
    }
}

#[derive(Serialize, Deserialize)]
struct EnumSetRaw {
    map: Vec<u32>,
}

impl<K> Serialize for EnumSet<K>
where
    K: Into<u32> + Copy + Ord,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let raw = self.to_raw();
        raw.serialize(serializer)
    }
}

impl<'de, K> Deserialize<'de> for EnumSet<K>
where
    K: TryFrom<u32> + Serialize + Ord,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(
            Self::try_from_raw(EnumSetRaw::deserialize(deserializer)?).map_err(|e| {
                serde::de::Error::custom(&format!("Failed to deserialize EnumSet: {e:?}"))
            })?,
        )
    }
}

#[derive(Serialize, Deserialize)]
struct EnumMapRaw {
    map: EnumSetRaw,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

impl<K, V> Serialize for EnumMap<K, V>
where
    K: Into<u32> + Copy + Ord,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let raw = self.to_raw();
        raw.serialize(serializer)
    }
}

impl<'de, K, V> Deserialize<'de> for EnumMap<K, V>
where
    K: TryFrom<u32> + Serialize + Ord,
    V: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(
            Self::try_from_raw(EnumMapRaw::deserialize(deserializer)?).map_err(|e| {
                serde::de::Error::custom(&format!("Failed to deserialize EnumMap: {e:?}"))
            })?,
        )
    }
}
