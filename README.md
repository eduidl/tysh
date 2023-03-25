# tysh

Tysh is th crate that helps with hashing the metadata of a type.

## Purpose

Tysh is a tool that is meant to be used with the [bincode](https://github.com/bincode-org/bincode) crate, which helps with serializing data. The bincode crate has some compatibility issues with certain data structures, such as the `Color { r: u8, g: u8, b: u8 }` and `Color { b: u8, g: u8, r: u8 }` structures, even though they have the same type structure and are compatible with JSON serialization. This is because bincode can change the meaning of the data when serializing and deserializing it.
To avoid this problem, tysh also provides field names along with the type information, which helps to ensure compatibility between different versions of data structures.

## How to use

```rust
use tysh::TypeHash;

#[derive(TypeHash)]
struct A {
    a: u8,
    b: u16,
}
```

This will generate the following code:

```rust
struct A {
    a: u8,
    b: u16,
}
impl ::tysh::TypeHash for A {
    fn type_hash<H: ::core::hash::Hasher>(hasher: &mut H) {
        use ::core::hash::Hash;
        "@struct@".hash(hasher);
        "A".hash(hasher);
        "@field@".hash(hasher);
        "a".hash(hasher);
        <u8 as ::tysh::TypeHash>::type_hash(hasher);
        "@field@".hash(hasher);
        "b".hash(hasher);
        <u16 as ::tysh::TypeHash>::type_hash(hasher);
    }
}
```

You can also use the `#[type_hash(name = "name")]` attribute to specify the internal name.

```rust
use tysh::TypeHash;

#[derive(TypeHash)]
#[type_hash(name = "AnotherName")]
struct A {
    a: u8,
    #[type_hash(name = "another special name")]
    b: u16,
}
```

```rust
pub struct A {
    a: u8,
    b: u16,
}
impl ::tysh::TypeHash for A {
    fn type_hash<H: ::core::hash::Hasher>(hasher: &mut H) {
        use ::core::hash::Hash;
        "@struct@".hash(hasher);
        "AnotherName".hash(hasher);
        "@field@".hash(hasher);
        "a".hash(hasher);
        <u8 as ::tysh::TypeHash>::type_hash(hasher);
        "@field@".hash(hasher);
        "another special name".hash(hasher);
        <u16 as ::tysh::TypeHash>::type_hash(hasher);
    }
}
```
