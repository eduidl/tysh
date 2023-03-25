#![cfg(feature = "std")]

use std::collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet, HashMap, HashSet};

use serde::Serialize;
use tysh::TypeHash;

fn convert<T, U>(from: &T) -> Result<U, bincode::Error>
where
    T: Serialize,
    U: for<'de> serde::Deserialize<'de>,
{
    let serialized = bincode::serialize(from).unwrap();
    bincode::deserialize(&serialized)
}

#[test]
fn test_set() {
    assert_eq!(
        HashSet::<i32>::type_hash_one::<DefaultHasher>(),
        BTreeSet::<i32>::type_hash_one::<DefaultHasher>()
    );

    let data = vec![1, 2, 3, 4];

    let set = data.iter().copied().collect();
    assert_eq!(
        convert::<HashSet<_>, BTreeSet<_>>(&set).unwrap(),
        data.into_iter().collect()
    );
}

#[test]
fn test_map() {
    assert_eq!(
        HashMap::<String, i32>::type_hash_one::<DefaultHasher>(),
        BTreeMap::<String, i32>::type_hash_one::<DefaultHasher>()
    );

    let data = vec![("hoge".into(), 2), ("fuga".into(), 4)];

    let map = data.iter().cloned().collect();
    assert_eq!(
        convert::<HashMap<String, _>, BTreeMap<_, _>>(&map).unwrap(),
        data.into_iter().collect()
    );
}

#[cfg(feature = "smallvec")]
#[test]
fn test_smallvec() {
    use smallvec::SmallVec;

    assert_eq!(
        SmallVec::<[u8; 4]>::type_hash_one::<DefaultHasher>(),
        Vec::<u8>::type_hash_one::<DefaultHasher>()
    );

    let data = vec![1, 2, 3, 4];

    let vec = SmallVec::<[u8; 4]>::from(data.clone());
    assert_eq!(convert::<_, Vec<u8>>(&vec).unwrap(), data);
}
