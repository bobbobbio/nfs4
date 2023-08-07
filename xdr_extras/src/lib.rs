// copyright 2023 Remi Bernotavicius

pub use xdr_extras_derive::*;

pub mod list {
    use serde::ser::SerializeStruct as _;
    use serde::{
        de::{DeserializeOwned, Deserializer},
        ser::Serializer,
        Deserialize, Serialize,
    };

    pub fn serialize<T, S>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("List", value.len() + 1)?;
        for v in value {
            state.serialize_field("entry", &Some(v))?;
        }

        state.serialize_field("end", &None::<T>)?;
        state.end()
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
    {
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("List")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut res = vec![];
                while let Some(v) = seq.next_element::<Option<T>>()?.flatten() {
                    res.push(v);
                }
                Ok(res)
            }
        }

        deserializer.deserialize_struct("List", &[], Visitor(std::marker::PhantomData))
    }
}
