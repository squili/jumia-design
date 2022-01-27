# Commands

In Jumia, application commands are a first class feature. You can easily
register commands using builder patterns and set callbacks for them.

### Example

```rust
async fn example() {
    Client::builder()
        .commands(|builder| {
            builder
                .add(|builder| {
                    builder
                        .name("echo")
                        .option(|builder| {
                            builder
                                .name("text")
                                .kind(OptionKind::String)
                        })
                        .callback(async |client, interaction, args| {
                            interaction.reply(args.get_string("text")).await.unwrap();
                        })
                })
        });
}
```

### Todo:
- Make it less wide
- Explore using macros
- Flesh out callbacks
