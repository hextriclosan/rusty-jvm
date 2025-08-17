use dashmap::mapref::one::{Ref, RefMut};
use std::hash::Hash;

/// A thread-safe map that automatically assigns keys to inserted values.
/// The keys are auto-incremented integers starting from a specified value.
/// This is useful for scenarios where you need unique identifiers for values
/// without managing the keys manually.
pub trait AutoDashMap<V> {
    /// The type of the key
    type Key: Copy + Eq + Hash;

    /// Creates a new `AutoDashMap` starting with the specified key.
    fn new(start_from: Self::Key) -> Self;

    /// Inserts a value with the next auto-increment key.
    /// Returns the key that was used.
    fn insert_auto(&self, value: V) -> Self::Key;

    /// Gets a reference to the value associated with the given key.
    /// Returns `None` if the key does not exist.
    fn get(&self, key: Self::Key) -> Option<Ref<Self::Key, V>>;

    /// Gets a mutable reference to the value associated with the given key.
    /// Returns `None` if the key does not exist.
    fn get_mut(&self, key: Self::Key) -> Option<RefMut<Self::Key, V>>;

    /// Removes the value associated with the given key and returns it.
    /// If the key does not exist, returns `None`.
    fn remove(&self, key: Self::Key) -> Option<V>;
}
