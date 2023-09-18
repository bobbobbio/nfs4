// copyright 2023 Remi Bernotavicius

use xdr_extras::{DeserializeWithDiscriminant, SerializeWithDiscriminant};

#[derive(DeserializeWithDiscriminant, SerializeWithDiscriminant, Debug, PartialEq)]
#[repr(u32)]
enum Foo {
    A = 1,
    B = 2,
    C = 3,
}

fn serialize_round_trip<
    V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
>(
    v: V,
    bytes: &[u8],
) {
    let new_v: V = serde_xdr::from_bytes(bytes).unwrap();
    assert_eq!(&new_v, &v);

    let new_bytes = serde_xdr::to_bytes(&v).unwrap();
    assert_eq!(&new_bytes, bytes);
}

#[test]
fn round_trip_unit_enum_xdr() {
    serialize_round_trip(Foo::A, &[0x0, 0x0, 0x0, 0x1][..]);
}

#[derive(DeserializeWithDiscriminant, SerializeWithDiscriminant, Debug, PartialEq)]
#[repr(u32)]
enum Bar {
    A(String) = 1,
    B {
        a: i32,
        b: u32,
    } = 2,
    C(i32, u64) = 7,
    #[serde(other)]
    D = 9,
}

#[test]
fn round_trip_fielded_enum_xdr() {
    serialize_round_trip(
        Bar::A("hi".into()),
        &[0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x2, 0x68, 0x69, 0x0, 0x0][..],
    );

    serialize_round_trip(
        Bar::B { a: 7, b: 8 },
        &[0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x7, 0x0, 0x0, 0x0, 0x8][..],
    );

    serialize_round_trip(
        Bar::C(5, 0xFF),
        &[
            0x0, 0x0, 0x0, 0x7, 0x0, 0x0, 0x0, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xFF,
        ][..],
    );

    serialize_round_trip(Bar::D, &[0x0, 0x0, 0x0, 0x9][..]);

    let new_v: Bar = serde_xdr::from_bytes([0x0, 0x0, 0x0, 0xFF]).unwrap();
    assert_eq!(&new_v, &Bar::D);
}

#[derive(DeserializeWithDiscriminant, SerializeWithDiscriminant, Debug, PartialEq)]
#[repr(u32)]
enum Baz<T> {
    A(T) = 3,
    B = 9,
}

#[test]
fn round_trip_generic_enum_xdr() {
    serialize_round_trip(
        Baz::A(0xFFu32),
        &[0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0xFF][..],
    );

    serialize_round_trip(Baz::<u32>::B, &[0x0, 0x0, 0x0, 0x9][..]);
}
