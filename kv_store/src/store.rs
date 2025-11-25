use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Default, Debug)]
pub struct KvStore {
    pub(crate) map: Mutex<HashMap<String, String>>,
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

    pub async fn set(&self, key: String, value: String) -> anyhow::Result<()> {
        {
            let mut map = self.map.lock().await;
            map.insert(key, value);
        }
        // call lock inside a block to release the lock before saving, and avoid deadlock
        // especially around await points
        // save also needs to acquire the lock, so deadlock would occur if we don't release it first
        self.save("data.json").await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<bool, anyhow::Error> {
       let removed = {
            let mut map = self.map.lock().await;
            map.remove(key).is_some()
       };
        if removed {
            self.save("data.json").await?;
        }
        Ok(removed)
    }
}