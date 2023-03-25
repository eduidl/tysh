#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};
use core::{
    cell::{Cell, RefCell},
    hash::Hasher,
    mem::MaybeUninit,
    num::Wrapping,
    sync::atomic,
};
#[cfg(feature = "std")]
use std::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use crate::TypeHash;

/// This trait is a marker trait for types that have same type hash as `Target`.
pub trait SameTypeHash {
    /// The type that has same type hash as `Self`.
    type Target: TypeHash;
}

impl<T: SameTypeHash> TypeHash for T {
    fn type_hash<H: Hasher>(hasher: &mut H) {
        <Self as SameTypeHash>::Target::type_hash(hasher);
    }
}

macro_rules! impl_same_type_hash {
    ($t:ty, $u:ty) => {
        impl SameTypeHash for $t {
            type Target = $u;
        }
    };
}

impl_same_type_hash!(atomic::AtomicBool, bool);
impl_same_type_hash!(atomic::AtomicI8, i8);
impl_same_type_hash!(atomic::AtomicI16, i16);
impl_same_type_hash!(atomic::AtomicI32, i32);
impl_same_type_hash!(atomic::AtomicI64, i64);
impl_same_type_hash!(atomic::AtomicU8, u8);
impl_same_type_hash!(atomic::AtomicU16, u16);
impl_same_type_hash!(atomic::AtomicU32, u32);
impl_same_type_hash!(atomic::AtomicU64, u64);

impl<T> SameTypeHash for Wrapping<T>
where
    T: TypeHash,
{
    type Target = T;
}

#[cfg(all(feature = "smallvec", feature = "alloc"))]
/// This can be used when the `smallvec` and `alloc` features are enabled.
impl<T> SameTypeHash for smallvec::SmallVec<T>
where
    T: smallvec::Array,
    T::Item: TypeHash,
{
    type Target = Vec<T::Item>;
}

#[cfg(feature = "std")]
/// This can be used when the `std` feature is enabled.
impl<T: TypeHash> SameTypeHash for BTreeSet<T> {
    type Target = HashSet<T>;
}

#[cfg(feature = "std")]
/// This can be used when the `std` feature is enabled.
impl<T: TypeHash, U: TypeHash> SameTypeHash for BTreeMap<T, U> {
    type Target = HashMap<T, U>;
}

impl<T: TypeHash> SameTypeHash for MaybeUninit<T> {
    type Target = T;
}

impl<T: TypeHash> SameTypeHash for RefCell<T> {
    type Target = T;
}

impl<T: TypeHash> SameTypeHash for Cell<T> {
    type Target = T;
}

#[cfg(feature = "alloc")]
/// This can be used when the `alloc` feature is enabled.
impl<T: TypeHash> SameTypeHash for Box<T> {
    type Target = T;
}

#[cfg(feature = "alloc")]
/// This can be used when the `alloc` feature is enabled.
impl<T: TypeHash> SameTypeHash for Rc<T> {
    type Target = T;
}

#[cfg(feature = "alloc")]
/// This can be used when the `alloc` feature is enabled.
impl<T: TypeHash> SameTypeHash for Arc<T> {
    type Target = T;
}

#[cfg(feature = "std")]
impl<T: TypeHash> SameTypeHash for Mutex<T> {
    type Target = T;
}

#[cfg(feature = "std")]
impl<T: TypeHash> SameTypeHash for RwLock<T> {
    type Target = T;
}
