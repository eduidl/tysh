#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![warn(clippy::all, clippy::nursery, missing_docs)]

#[cfg(all(feature = "smallvec", not(feature = "alloc")))]
compile_error!("smallvec feature requires std or alloc feature");

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "int128")]
use core::num::{NonZeroI128, NonZeroU128};
use core::{
    hash::{Hash, Hasher},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64,
        NonZeroU8,
    },
};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

mod same_type;

pub use same_type::SameTypeHash;
pub use tysh_derive::TypeHash;

const PRIMITIVE: &str = "@primitive@";
const STANDARD: &str = "@standard@";

/// A type for which its metadata is hashable.
///
/// Consider using `#[derive(`[`TypeHash`](derive.TypeHash.html)`)]` before implementing this trait directly.
pub trait TypeHash {
    /// Hashes the type metadata.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::hash_map::DefaultHasher;
    ///
    /// # use tysh::TypeHash;
    /// #
    /// #[derive(TypeHash)]
    /// pub struct A {
    ///     a: u8,
    ///     b: u16,
    /// }
    ///
    /// let mut hasher = DefaultHasher::new();
    /// A::type_hash(&mut hasher);
    /// ```
    fn type_hash<H: Hasher>(hasher: &mut H);

    /// Returns a hash value of the type metadata.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std::collections::hash_map::DefaultHasher;
    ///
    /// # use tysh::TypeHash;
    /// #
    /// #[derive(TypeHash)]
    /// pub struct A {
    ///     a: u8,
    ///     b: u16,
    /// }
    ///
    /// dbg!(A::type_hash_one::<DefaultHasher>());
    /// ```
    fn type_hash_one<H: Default + Hasher>() -> u64 {
        let mut hasher = H::default();
        Self::type_hash(&mut hasher);
        hasher.finish()
    }
}

macro_rules! impl_for_primitive {
    ($t:ty) => {
        impl TypeHash for $t {
            fn type_hash<H: Hasher>(hasher: &mut H) {
                PRIMITIVE.hash(hasher);
                stringify!($t).hash(hasher);
            }
        }
    };
}

impl_for_primitive!(u8);
impl_for_primitive!(u16);
impl_for_primitive!(u32);
impl_for_primitive!(u64);
#[cfg(feature = "int128")]
/// This can be used when the `int128` feature is enabled.
impl TypeHash for u128 {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        PRIMITIVE.hash(hasher);
        stringify!(u128).hash(hasher);
    }
}
impl_for_primitive!(i8);
impl_for_primitive!(i16);
impl_for_primitive!(i32);
impl_for_primitive!(i64);
#[cfg(feature = "int128")]
/// This can be used when the `int128` feature is enabled.
impl TypeHash for i128 {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        PRIMITIVE.hash(hasher);
        stringify!(i128).hash(hasher);
    }
}
impl_for_primitive!(f32);
impl_for_primitive!(f64);
impl_for_primitive!(bool);
impl_for_primitive!(char);

impl_for_primitive!(NonZeroU8);
impl_for_primitive!(NonZeroU16);
impl_for_primitive!(NonZeroU32);
impl_for_primitive!(NonZeroU64);
#[cfg(feature = "int128")]
/// This can be used when the `int128` feature is enabled.
impl TypeHash for NonZeroU128 {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        PRIMITIVE.hash(hasher);
        stringify!(NonZeroU128).hash(hasher);
    }
}
impl_for_primitive!(NonZeroI8);
impl_for_primitive!(NonZeroI16);
impl_for_primitive!(NonZeroI32);
impl_for_primitive!(NonZeroI64);
#[cfg(feature = "int128")]
/// This can be used when the `int128` feature is enabled.
impl TypeHash for NonZeroI128 {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        PRIMITIVE.hash(hasher);
        stringify!(NonZeroI128).hash(hasher);
    }
}

#[cfg(feature = "alloc")]
/// This can be used when the `alloc` feature is enabled.
impl TypeHash for String {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        STANDARD.hash(hasher);
        stringify!(String).hash(hasher);
    }
}

impl<T: TypeHash> TypeHash for Option<T> {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        STANDARD.hash(hasher);
        "Option".hash(hasher);
        T::type_hash(hasher);
    }
}

impl<T: TypeHash, E: TypeHash> TypeHash for Result<T, E> {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        STANDARD.hash(hasher);
        "Result".hash(hasher);
        T::type_hash(hasher);
        E::type_hash(hasher);
    }
}

impl<const N: usize, T: TypeHash> TypeHash for [T; N] {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        STANDARD.hash(hasher);
        "Array".hash(hasher);
        N.hash(hasher);
        T::type_hash(hasher);
    }
}

#[cfg(feature = "alloc")]
/// This can be used when the `alloc` feature is enabled.
impl<T: TypeHash> TypeHash for Vec<T> {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        STANDARD.hash(hasher);
        "Vec".hash(hasher);
        T::type_hash(hasher);
    }
}

#[cfg(feature = "std")]
/// This can be used when the `std` feature is enabled.
impl<T: TypeHash> TypeHash for HashSet<T> {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        STANDARD.hash(hasher);
        "Set".hash(hasher);
        T::type_hash(hasher);
    }
}

#[cfg(feature = "std")]
/// This can be used when the `std` feature is enabled.
impl<T: TypeHash, U: TypeHash> TypeHash for HashMap<T, U> {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        STANDARD.hash(hasher);
        "Map".hash(hasher);
        T::type_hash(hasher);
        U::type_hash(hasher);
    }
}

macro_rules! impl_for_tuple {
    ($($xs:ident),+ $(,)?) => {
        impl<$($xs),+> TypeHash for ($($xs),+)
        where
            $($xs: TypeHash),+
        {
            fn type_hash<H: Hasher>(hasher: &mut H) {
               STANDARD.hash(hasher);
                "Tuple".hash(hasher);
                $(
                    $xs::type_hash(hasher);
                )+
            }
        }
    };
}

impl<T1: TypeHash> SameTypeHash for (T1,) {
    type Target = T1;
}

impl_for_tuple!(T1, T2);
impl_for_tuple!(T1, T2, T3);
impl_for_tuple!(T1, T2, T3, T4);
impl_for_tuple!(T1, T2, T3, T4, T5);
impl_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
