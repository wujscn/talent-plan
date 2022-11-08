use std::collections::HashMap;

/// KvStore is a pretty simple HassMap for String
/// 
/// No more comments here, just for test
/// 
/// ```rust
/// fn prelude() {
///     println!("Hello, world!");
/// }
/// ```
pub struct KvStore {
    hashmap: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            hashmap: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.hashmap.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
        // match self.hashmap.get(&key) {
        //     Some(v) => Some(v.to_string()),
        //     None => None,
        // }
        self.hashmap.get(&key).map(|v| v.to_string())
    }

    pub fn remove(&mut self, key: String) {
        self.hashmap.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
