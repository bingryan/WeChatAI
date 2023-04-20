use serde_json::value::Value;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;

// KVStorage trait
pub trait KVStorage {
    fn add(&mut self, key: &str, value: Value) -> anyhow::Result<()>;
    fn remove(&mut self, key: &str) -> anyhow::Result<()>;
    fn query(&self, key: &str) -> anyhow::Result<Option<Value>>;
    fn update(&mut self, key: &str, value: Value) -> anyhow::Result<()>;
    fn save(&mut self) -> anyhow::Result<()>;
    fn batch_update(&mut self, data: HashMap<String, Value>) -> anyhow::Result<()>;
}

// LocalJsonStorage
pub struct LocalJsonStorage {
    path: PathBuf,
    pub data: HashMap<String, Value>,
}

impl LocalJsonStorage {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        // if the path is not existing, error
        let mut file = std::fs::File::open(path.clone())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let data: HashMap<String, Value> = serde_json::from_str(&content)?;
        Ok(Self { path, data })
    }

    pub fn create(path: PathBuf, content: Option<String>, force: bool) -> anyhow::Result<Self> {
        // if the path is existing, error
        if !force && path.exists() {
            return Err(anyhow::anyhow!("path is existing"));
        }
        let mut data: HashMap<String, Value> = HashMap::new();
        if let Some(content) = content {
            data = serde_json::from_str(&content)?;
        }
        // if parent directory is not existing, create it
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        // force create the file, and write the content
        let mut file = std::fs::File::create(path.clone())?;
        let content = serde_json::to_string_pretty(&data)?;
        file.write_all(content.as_bytes())?;
        Ok(Self { path, data })
    }
}

impl KVStorage for LocalJsonStorage {
    fn add(&mut self, key: &str, value: Value) -> anyhow::Result<()> {
        self.data.insert(key.to_string(), value);
        self.save()
    }

    fn remove(&mut self, key: &str) -> anyhow::Result<()> {
        self.data.remove(key);
        self.save()
    }

    fn query(&self, key: &str) -> anyhow::Result<Option<Value>> {
        Ok(self.data.get(key).cloned())
    }

    fn update(&mut self, key: &str, value: Value) -> anyhow::Result<()> {
        self.data.insert(key.to_string(), value);
        self.save()
    }

    fn save(&mut self) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(self.path.clone())?;
        let content = serde_json::to_string_pretty(&self.data)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn batch_update(&mut self, data: HashMap<String, Value>) -> anyhow::Result<()> {
        for (key, value) in data {
            self.data.insert(key, value);
        }
        let mut file = std::fs::File::create(self.path.clone())?;
        let content = serde_json::to_string_pretty(&self.data)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
