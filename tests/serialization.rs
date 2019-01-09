extern crate rand;
extern crate bn;
extern crate bincode;
extern crate serde;
extern crate hex;
#[macro_use] extern crate lazy_static;

mod test_data;
use bn::*;

use hex::{ToHex, FromHex};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub fn into_hex<S: Serialize>(obj: S) -> Option<String> {
    serialize(&obj).ok().map(hex::encode)
}

pub fn from_hex<S: DeserializeOwned>(s: &str) -> Result<S, bincode::Error> {
    let s = hex::decode(s).unwrap();

    deserialize(&s)
}

pub fn reserialize<S: Serialize + DeserializeOwned>(obj: S) -> S {
    let s = into_hex(obj).unwrap();

    from_hex(&s).unwrap()
}

#[test]
fn group_serialization_and_deserialization() {
    let b = Fr::random(&mut rand::thread_rng());

    let mut a = G1::one();
    for _ in 0..100 {
        a = a * b;

        assert!(reserialize(a) == a);
        assert!(reserialize(reserialize(a)) == a);
        let mut c = a;
        c.normalize();

        assert!(a == c);
    }

    let mut a = G2::one();
    for _ in 0..100 {
        a = a * b;

        assert!(reserialize(a) == a);
        assert!(reserialize(reserialize(a)) == a);
        let mut c = a;
        c.normalize();

        assert!(a == c);

    }
}

#[test]
fn group_serialization_edge_cases() {
    assert!(from_hex::<G1>("00").unwrap() == G1::zero());
    assert!(from_hex::<G2>("00").unwrap() == G2::zero());
    assert!(from_hex::<G1>("23").is_err());
    assert!(from_hex::<G2>("23").is_err());

    // not points on the curve
    assert!(from_hex::<G1>("04177cedb64589bde7a64ad24f89bbb8c9f05535810865aaea8fbf8184ff9e120313500226b2422d2068614d1c8c7146c806a97743e78d9901748a9ded08ea9e5f").is_err());
    assert!(from_hex::<G2>("0404d4bf3239f77cee7b47c7245e9280b3e9c1182d6381a87bbf81f9f2a6254b731df569cda95e060bee91ba69b3f2d103658a7aea6b10e5bdc761e5715e7ee4bb01b4c328f0cbdb4aada63b3d09100d792376b94d07a6004e46054eeec849e8de9835158a11d28483dd8db236ea49f3630edc9e41944e494c5aacfc36af3b66e7").is_err());

    // out of bounds
    assert!(from_hex::<G2>("04ffd6a64a62b843a22c6250eda2354d603e74c30ed0b1435951c3f7dd541538beb8a43915823125c6bb89aece89125664ce78ca69b81cdb8164b40eb2833560b606e11258ce33c4076eff0d5824f210466b588d324b60ccd5a2b7f180f9a7cd7f1ab564ddb03b1b684ff4315acc6eef5229d99fe107afaea83a5c72f2b4c33aca").is_err());
}

#[test]
fn g1_vectors() {
    let expected = &test_data::G1_VECTORS_EXPECTED;

    let mut acc = G1::one();
    let by = Fr::from_str("23938123").unwrap();

    for i in 0..10000 {
        assert!(expected[i] == into_hex(acc).unwrap());
        assert!(from_hex::<G1>(expected[i]).unwrap() == acc);

        acc = acc * by + acc;
    }
}

#[test]
fn g2_vectors() {
    let expected = &test_data::G2_VECTORS_EXPECTED;

    let mut acc = G2::one();
    let by = Fr::from_str("23938123").unwrap();

    for i in 0..10000 {
        assert!(expected[i] == into_hex(acc).unwrap());
        assert!(from_hex::<G2>(expected[i]).unwrap() == acc);

        acc = acc * by + acc;
    }
}

#[test]
fn fr_serialization_and_deserialization() {
    let mut a = Fr::one();
    let b = Fr::from_str("17").unwrap();

    for _ in 0..10000 {
        a = a * b;

        assert!(reserialize(a) == a);
    }
}

#[test]
fn fr_test_invalid_representations() {
    // modulus - 1 is representable
    from_hex::<Fr>("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000").unwrap();

    // modulus is not representable
    assert!(from_hex::<Fr>("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001").is_err());

    // ridiculously large numbers
    assert!(from_hex::<Fr>("f0644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001").is_err());
    assert!(from_hex::<Fr>("ffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000").is_err());
}

#[test]
fn fr_vectors() {
    let expected = &test_data::FR_VECTORS_EXPECTED;

    let mut acc = Fr::one();
    for i in 0..10000 {
        assert!(expected[i] == into_hex(acc).unwrap());
        assert!(from_hex::<Fr>(expected[i]).unwrap() == acc);

        acc = acc * acc + acc + acc.inverse().unwrap();
    }
}
