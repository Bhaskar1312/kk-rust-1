// use std::fs;
use tokio::fs;
use std::path::Path;
use crate::store::KvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PersistentData {
    data: Vec<(String, String)>,
}

impl KvStore {
    pub async fn load(&self, path: &str) -> anyhow::Result<()> {
        if Path::new(path).exists() {
            let content = tokio::fs::read_to_string(path).await?;
            let persistent_data: PersistentData = serde_json::from_str(&content)?;
            let mut map = self.map.lock().await;
            map.extend(persistent_data.data);
        }
        Ok(())
    }

    pub async fn save(&self, path: &str) -> anyhow::Result<()> {
        let data = {
            let map = self.map.lock().await;
            PersistentData {
                data: map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            }
        }; // lock is released here

        let content = serde_json::to_string(&data)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
}