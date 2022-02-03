# Actions

Actions are how you do things in Jumia. They must implement the `Action` trait.

### Trait
```rust
trait Action {
    type Output;

    // Does the actual change without checking if it can
    async fn execute(self, client: &Client) -> Result<Self::Output, Error>;
    
    // Check if the action can be carried out. This may not be valid in the
    // future, since we can't stop Discord from doing stuff while we process
    async fn check(self, client: &Client) -> Result<Self, Error>;
    
    // Check and then execute the action. This is the recommended way of running actions
    async fn run(self, client: &Client) -> Result<Self::Output, Error> {
        self.check(client).await?.execute(client).await
    }
}
```

### Example
```rust
async fn example(client: &Client) {
    let _new_channel = CreateGuildChannel::category()
        .name("Category Channel")
        .create_channel(CreateGuildChannel::text().name("text-channel"))
        .run(client)
        .await
        .unwrap();
}
```

### Notes

- Nested executions should be run in parallel when possible
