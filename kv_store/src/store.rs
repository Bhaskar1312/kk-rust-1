use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Default, Debug)]
pub struct KvStore {
    map: Mutex<HashMap<String, String>>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let map = self.map.lock().await;
        map.get(key).cloned()
    }

    pub async fn set(&self, key: String, value: String) {
        let mut map = self.map.lock().await;
        map.insert(key, value);
    }

    pub async fn delete(&self, key: &str) -> bool {
        let mut map = self.map.lock().await;
        map.remove(key).is_some()
    }
}