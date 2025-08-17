use crate::vm::commons::auto_dash_map::auto_dash_map::AutoDashMap;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use serde::ser::SerializeMap;
use serde::Serialize;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct AutoDashMapI32<V> {
    map: DashMap<i32, V>,
    counter: Arc<AtomicI32>,
}

impl<V> Default for AutoDashMapI32<V> {
    fn default() -> Self {
        Self {
            map: DashMap::default(),
            counter: Arc::new(AtomicI32::default()),
        }
    }
}

impl<V> Serialize for AutoDashMapI32<V>
where
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = self.map.iter().collect::<Vec<_>>(); // snapshot
        let mut ser_map = serializer.serialize_map(Some(map.len()))?;
        for entry in map {
            ser_map.serialize_entry(entry.key(), entry.value())?;
        }
        ser_map.end()
    }
}

impl<V> AutoDashMap<V> for AutoDashMapI32<V> {
    type Key = i32;

    fn new(start_from: Self::Key) -> Self {
        Self {
            map: DashMap::new(),
            counter: Arc::new(AtomicI32::new(start_from)),
        }
    }

    fn insert_auto(&self, value: V) -> Self::Key {
        let key = self.counter.fetch_add(1, Ordering::Relaxed); // relaxed is sufficient here
        self.map.insert(key, value);
        key
    }

    fn get(&self, key: Self::Key) -> Option<Ref<Self::Key, V>> {
        self.map.get(&key)
    }

    fn get_mut(&self, key: Self::Key) -> Option<RefMut<Self::Key, V>> {
        self.map.get_mut(&key)
    }

    fn remove(&self, key: Self::Key) -> Option<V> {
        self.map.remove(&key).map(|entry| entry.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn should_create_default_map() {
        let map = AutoDashMapI32::default();
        let key1 = map.insert_auto("value0");
        let key2 = map.insert_auto("value1");
        assert_eq!(key1, 0);
        assert_eq!(key2, 1);
    }

    #[test]
    fn should_insert_entry() {
        let map = AutoDashMapI32::new(1000);
        let key1 = map.insert_auto("value1000");
        let key2 = map.insert_auto("value1001");
        assert_eq!(key1, 1000);
        assert_eq!(key2, 1001);
    }

    #[test]
    fn should_return_const_value() {
        let map = AutoDashMapI32::new(1000);
        let key = map.insert_auto("value1000");
        let value = map.get(key).unwrap();
        assert_eq!(*value, "value1000");
    }

    #[test]
    fn should_mutate_value() {
        let map = AutoDashMapI32::new(1);
        let key = map.insert_auto("value");
        {
            let mut value = map.get_mut(key).unwrap();
            assert_eq!(*value, "value");
            *value = "new_value";
        }
        let modified_value = map.get(key).unwrap();
        assert_eq!(*modified_value, "new_value");
    }

    #[test]
    fn should_remove_entry() {
        let map = AutoDashMapI32::new(1);
        let key = map.insert_auto("value");
        let removed_value = map.remove(key).unwrap();
        assert_eq!(removed_value, "value");
        assert!(map.get(key).is_none());
    }

    #[test]
    fn should_serialize() {
        let map = AutoDashMapI32::new(1);
        map.insert_auto("value1");
        map.insert_auto("value2");
        let serialized = serde_json::to_string(&map).unwrap();
        assert!(serialized.contains("\"1\":\"value1\""));
        assert!(serialized.contains("\"2\":\"value2\""));
    }

    #[test]
    fn should_return_none_on_getting_none_existing_entry() {
        let map = AutoDashMapI32::new(1);
        map.insert_auto("value");
        assert!(map.get(999).is_none());
    }

    #[test]
    fn should_return_none_on_removing_none_existing_entry() {
        let map = AutoDashMapI32::new(1);
        map.insert_auto("value");
        assert!(map.remove(999).is_none());
    }
}
