extern crate serde_rlp;
#[macro_use]
extern crate serde_derive;

use serde_rlp::ser::to_bytes;

#[derive(Debug, Serialize)]
struct Person {
    first_name: String,
    last_name: String,
    age: u64,
}

#[test]
fn serialize() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 42u64,
    };
    // Generated with pyrlp with [["first_name", "John"], ["last_name", "Doe"], ["age", 42]]
    assert_eq!(
        to_bytes(&person).unwrap(),
        vec![
            0xe6, 0xd0, 0x8a, 0x66, 0x69, 0x72, 0x73, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x84,
            0x4a, 0x6f, 0x68, 0x6e, 0xce, 0x89, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x6e, 0x61, 0x6d,
            0x65, 0x83, 0x44, 0x6f, 0x65, 0xc5, 0x83, 0x61, 0x67, 0x65, 0x2a,
        ]
    );
}
