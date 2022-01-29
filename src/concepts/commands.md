# Commands

###### Draft - consider alternate ideas

In Jumia, application commands are a first class feature. You can easily register commands using builder patterns and
set callbacks for them.

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
                            interaction.reply(client, args.get_string("text")).await.unwrap();
                            Ok(())
                        })
                })
        });
}
```

### Callback template

```rust
async fn callback(_client: Client, _interaction: Interaction, _args: CommandArgs) -> Result<(), JumiaError> {}
```

### Todo:
- Make it less wide
- Should we be using a result type
- Can we support callbacks that don't have args (trait bullshit?)

## More ideas

### Macro builder

Talk to Julia about her design

### Derive macros

Look at serenity's framework system
- Hard
- Future goal

### Pure router

Instead of defining the commands and callbacks in the same place, just have a router
- Easier middleware support
- Can steal designs from warp/axum, maybe even tower
- Separating the two could be harder to reason about
  - Can't easily check stuff at runtime
  - Need to add stuff to two different locations to add command

### Wrappers for middleware

Instead of doing fancy middleware stuff using a builder pattern, you can use something like
`GuildOnly::wrap(callback)` or `guild_only(callback)`
- Should be fairly easy to do
- Natural for people used to functional programming
  - Unnatural for others
- Could end up repetitive

### Permissions

While discord's new permissions system is fancy, it doesn't have all the stuff some bots need. For example, you can't
have subcommand-specific permissions. The problem is that we would need the developer to provide some custom
implementation for persisting permissions data - and I definitely don't want to solve this by adding SQLx to the
crate dependencies
