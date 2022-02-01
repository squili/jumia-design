use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;
use thiserror::Error;

trait StorageKey {
    const ID: &'static str;
    type Value: DeserializeOwned + Serialize + Send;

    fn get_key(self) -> String;
}

#[async_trait]
trait StorageBackend: Sized {
    type Error: std::error::Error;

    async fn register<K: StorageKey + Send>(&self) -> Result<(), Self::Error>;
    async fn get<K: StorageKey + Send>(&self, key: K) -> Result<Option<K::Value>, Self::Error>;
    async fn set<K: StorageKey + Send>(&self, key: K, value: K::Value) -> Result<(), Self::Error>;
}

#[derive(Error, Debug)]
enum JsonStorageError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Attempted to use unregistered id: {0}")]
    UnregisteredId(&'static str),
}

struct JsonStorage {
    path: Box<Path>,
    data: Mutex<HashMap<String, HashMap<String, serde_json::Value>>>,
}

impl JsonStorage {
    async fn new(path: impl AsRef<Path>) -> Result<Self, JsonStorageError> {
        let path = path.as_ref();

        let data = if path.exists() {
            serde_json::from_str(&tokio::fs::read_to_string(path).await?)?
        } else {
            Default::default()
        };

        Ok(Self {
            path: Box::from(path),
            data: Mutex::new(data),
        })
    }

    async fn flush(&self) -> Result<(), JsonStorageError> {
        let data = serde_json::to_string(self.data.lock().await.deref())?;
        tokio::fs::write(&self.path, data).await?;

        Ok(())
    }
}

#[async_trait]
impl StorageBackend for JsonStorage {
    type Error = JsonStorageError;

    async fn register<K: StorageKey + Send>(&self) -> Result<(), Self::Error> {
        self.data.lock().await.entry(K::ID.to_string()).or_insert_with(Default::default);

        Ok(())
    }

    async fn get<K: StorageKey + Send>(&self, key: K) -> Result<Option<K::Value>, Self::Error> {
        let handle = self.data.lock().await;
        let map = match handle.get(K::ID) {
            Some(s) => s,
            None => return Err(JsonStorageError::UnregisteredId(K::ID)),
        };

        Ok(match map.get(&key.get_key()) {
            Some(s) => Some(serde_json::from_value(s.clone())?),
            None => None,
        })
    }

    async fn set<K: StorageKey + Send>(&self, key: K, value: K::Value) -> Result<(), Self::Error> {
        let mut handle = self.data.lock().await;
        let map = match handle.get_mut(K::ID) {
            Some(s) => s,
            None => return Err(JsonStorageError::UnregisteredId(K::ID)),
        };

        map.insert(key.get_key(), serde_json::to_value(value)?);
        drop(handle);

        self.flush().await
    }
}

struct TestKey(String);

#[derive(Deserialize, Serialize, Debug)]
struct TestValue {
    foo: String,
}

impl StorageKey for TestKey {
    const ID: &'static str = "testkey";
    type Value = TestValue;

    fn get_key(self) -> String {
        self.0
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {
        // store data into storage
        println!("opening");
        let storage = JsonStorage::new("test_storage.json").await?;
        println!("registering");
        storage.register::<TestKey>().await?;

        let key = TestKey("ThisIsTheKey".to_string());
        let value = TestValue {
            foo: "Hello world!".to_string()
        };

        println!("setting");
        storage.set(key, value).await?;
    }

    println!("\n-----\n");

    {
        // load data from storage
        println!("opening");
        let storage = JsonStorage::new("test_storage.json").await?;
        println!("registering");
        storage.register::<TestKey>().await?;

        println!("getting");
        println!("stored value: {:?}", storage.get(TestKey("ThisIsTheKey".to_string())).await?);
        println!("non-stored value: {:?}", storage.get(TestKey("ThisIsNotTheKey".to_string())).await?);
    }

    Ok(())
}
