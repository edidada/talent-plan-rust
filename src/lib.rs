#![deny(missing_docs)]

//! An in-memory key/value store used throughout the PNA Rust course.

use std::collections::HashMap;

/// A simple key/value store that keeps all data in memory.
///
/// Values are lost when the store is dropped. Use [`KvStore::set`] to insert or
/// overwrite an entry, [`KvStore::get`] to read it, and [`KvStore::remove`] to
/// delete it.
///
/// # Examples
///
/// ```
/// use kvs::KvStore;
///
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// assert_eq!(store.get("key".to_owned()), Some("value".to_owned()));
/// store.remove("key".to_owned());
/// assert_eq!(store.get("key".to_owned()), None);
/// ```
pub struct KvStore {
    map: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Creates an empty store.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let store = KvStore::new();
    /// assert_eq!(store.get("missing".to_owned()), None);
    /// ```
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Inserts a key/value pair, replacing any previous value for `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("a".to_owned(), "1".to_owned());
    /// store.set("a".to_owned(), "2".to_owned());
    /// assert_eq!(store.get("a".to_owned()), Some("2".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Returns the value for `key`, or `None` when the key is absent.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// assert_eq!(store.get("a".to_owned()), None);
    /// store.set("a".to_owned(), "1".to_owned());
    /// assert_eq!(store.get("a".to_owned()), Some("1".to_owned()));
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Removes `key` from the store. Removing a missing key does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("a".to_owned(), "1".to_owned());
    /// store.remove("a".to_owned());
    /// assert_eq!(store.get("a".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
