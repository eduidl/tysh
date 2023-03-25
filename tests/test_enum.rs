#![allow(dead_code)]
#![cfg(feature = "std")]

use std::collections::hash_map::DefaultHasher;

use tysh::TypeHash;

mod original {
    use super::*;

    #[derive(TypeHash)]
    pub enum Enum {
        A(u8, String),
        B { x: u16, y: Vec<f32> },
        C,
    }
}

#[test]
fn test_type_hash_matches() {
    #[derive(TypeHash)]
    enum Enum {
        A(u8, String),
        B { x: u16, y: Vec<f32> },
        C,
    }

    assert_eq!(
        original::Enum::type_hash_one::<DefaultHasher>(),
        Enum::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_not_match_if_different_enum_name() {
    #[derive(TypeHash)]
    enum Enum2 {
        A(u8, String),
        B { x: u16, y: Vec<f32> },
        C,
    }

    assert_ne!(
        original::Enum::type_hash_one::<DefaultHasher>(),
        Enum2::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_not_match_if_different_field_name() {
    #[derive(TypeHash)]
    enum Enum {
        Changed(u8, String),
        B { x: u16, y: Vec<f32> },
        C,
    }

    assert_ne!(
        original::Enum::type_hash_one::<DefaultHasher>(),
        Enum::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_matches_inner_type_name() {
    #[derive(TypeHash)]
    #[type_hash(name = "Enum")]
    enum Enum2 {
        A(u8, String),
        B { x: u16, y: Vec<f32> },
        C,
    }

    assert_eq!(
        original::Enum::type_hash_one::<DefaultHasher>(),
        Enum2::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_matches_inner_variant_name() {
    #[derive(TypeHash)]
    enum Enum {
        #[type_hash(name = "A")]
        Changed(u8, String),
        #[type_hash(name = "B")]
        Changed2 {
            x: u16,
            y: Vec<f32>,
        },
        C,
    }

    assert_eq!(
        original::Enum::type_hash_one::<DefaultHasher>(),
        Enum::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_matches_inner_field_name() {
    #[derive(TypeHash)]
    enum Enum {
        A(u8, String),
        B {
            #[type_hash(name = "x")]
            u: u16,
            #[type_hash(name = "y")]
            v: Vec<f32>,
        },
        C,
    }

    assert_eq!(
        original::Enum::type_hash_one::<DefaultHasher>(),
        Enum::type_hash_one::<DefaultHasher>()
    );
}
