# Storage

Jumia has a persistence system that allows pre-built systems to use persistent data without significant user
configuration. An example of this can be found
[in the examples](https://github.com/squili/jumia-design/blob/main/examples/storage/src/main.rs).

### Traits

There are two traits that the storage system uses. The storage backend must implement the `StorageBackend` trait. A
storage key must implement the `StorageKey` trait.

```rust
trait StorageKey {
    const ID: &'static str;
    type Value: DeserializeOwned + Serialize + Send;

    fn get_key(self) -> String;
}

trait StorageBackend: Sized {
    type Error: std::error::Error;

    async fn register<K: StorageKey + Send>(&self) -> Result<(), Self::Error>;
    async fn get<K: StorageKey + Send>(&self, key: K) -> Result<Option<K::Value>, Self::Error>;
    // return previous value, if any
    async fn set<K: StorageKey + Send>(&self, key: K, value: K::Value) -> Result<Option<K::Value>, Self::Error>;
}
```

### Provided

A user implementing this system, especially for a more complex backend, could take an evening. Instead, Jumia will
provide some default implementations - file and redis (and maybe etcd)

### Extensible

This extensible system means that anyone can write an addon system that will use a centralized persistent data system.
For example, a custom permission system could just need a `Permission` struct supplied with a storage backend and
provide a wrapper function. For the library user, this is very simple and easy. Another example is resumable sessions.
Currently, no library automatically supports resumable sessions, but it'd be really cool to automatically load those.
