# Storage

It would be nice to store some data persistently. There are three main options for this:

### 1. Redis

You supply the library with a link to redis, and it does everything on its own
- Requires users to run a redis server

### 2. Trait

You supply an implementation of the Storage trait. This trait probably needs someone else to look at it

```rust
enum StorageKey {
    Permission(String),
    Resumable([u64; 2]),
}

trait Storage {
    type Error: Error;
    async fn get(key: StorageKey) -> Result<&dyn Deserialize, Self::Error>;
    async fn set(key: StorageKey, value: &dyn Serialize) -> Result<(), Self::Error>;
    
    // for clearing out old data
    async fn delete(key: StorageKey) -> Result<(), Self::Error>;
    async fn iter() -> Result<HashMap<StorageKey, &dyn Deserialize>, Self::Error>;
}
```

- Very flexible
- Requires user to implement a storage backend
- Adding storage keys is a breaking change

### 3. File

Some sort of persistent local storage - like RON or Structsy - is used to save the data. This could also be supplied as
an additional crate that implements a storage backend for these types for users who don't want to implement the Storage
trait themselves
- Very easy for users
- Makes a random file on disc - might be confusing
