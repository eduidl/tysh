#![allow(dead_code)]
#![cfg(feature = "std")]

use std::collections::hash_map::DefaultHasher;

use tysh::TypeHash;

mod original {
    use super::*;

    #[derive(TypeHash)]
    pub struct A {
        a: u8,
        b: String,
        c: f32,
    }
}

#[test]
fn test_general_type_is_hashable() {
    #[derive(TypeHash)]
    struct A {
        a: u16,
        b: f32,
        c: bool,
        d: String,
        e: Vec<Vec<u32>>,
        f: Option<String>,
        g: Result<u8, String>,
        h: [String; 10],
    }

    A::type_hash_one::<DefaultHasher>();
}

#[test]
fn test_type_hash_matches() {
    #[derive(TypeHash)]
    struct A {
        a: u8,
        b: String,
        c: f32,
    }

    assert_eq!(
        original::A::type_hash_one::<DefaultHasher>(),
        A::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_not_match_if_different_struct_name() {
    #[derive(TypeHash)]
    struct B {
        a: u8,
        b: String,
        c: f32,
    }

    assert_ne!(
        original::A::type_hash_one::<DefaultHasher>(),
        B::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_not_match_if_different_field_name() {
    #[derive(TypeHash)]
    struct A {
        changed: u8,
        b: String,
        c: f32,
    }

    assert_ne!(
        original::A::type_hash_one::<DefaultHasher>(),
        A::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_matches_inner_type_name() {
    #[derive(TypeHash)]
    #[type_hash(name = "A")]
    struct B {
        a: u8,
        b: String,
        c: f32,
    }

    assert_eq!(
        original::A::type_hash_one::<DefaultHasher>(),
        B::type_hash_one::<DefaultHasher>()
    );
}

#[test]
fn test_type_hash_matches_inner_field_name() {
    #[derive(TypeHash)]
    struct A {
        #[type_hash(name = "a")]
        changed: u8,
        b: String,
        c: f32,
    }

    assert_eq!(
        original::A::type_hash_one::<DefaultHasher>(),
        A::type_hash_one::<DefaultHasher>()
    );
}
