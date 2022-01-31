# Commands

This is an example of an implementation of a command

```rust
// library code
trait CommandCallback<E: Into<JumiaError>> {
    async fn call(&self, client: &Client, interaction: &Interaction, args: &CommandArgs) -> Result<(), E>;
}

impl<E, F> CommandCallback<E> for F
where 
    E: Into<JumiaError>,
    F: Fn(&Client, &Interaction, &CommandArgs) -> Result<(), BotError>,
{
    async fn call(&self, client: &Client, interaction: &Interaction, args: &CommandArgs) -> Result<(), E> {
        self(client, interaction, args)
    }
}

// either library or user code
fn guild_only(callback: impl CommandCallback) -> impl CommandCallback {
    |client, interaction, args| async move {
        if interaction.guild_id.is_none() {
            interaction.reply(client, "Command must be run in guild").await?;
        } else {
            callback(client, interaction, args).await?;
        }
    }
}

// user code
async fn echo_command(client: Client, interaction: Interaction, args: CommandArgs) -> Result<(), BotError> {
    interaction.reply(client, args.get_string("text")).await?;
    Ok(())
}

fn example() {
    Client::builder()
        .commands(|commands| {
            commands
                .add(CommandBuilder::new()
                    .name("echo")
                    .option(OptionBuilder::new()
                        .name("text")
                        .kind(OptionKind::String))
                    .callback(guild_only(echo_command))
                )
        });
}
```

### Permissions

While discord's new permissions system is fancy, it doesn't have all the stuff some bots need. For example, you can't
have subcommand-specific permissions. The problem is that we would need the developer to provide some custom
implementation for persisting permissions data - and I definitely don't want to solve this by adding SQLx to the
crate dependencies. Julia provided this idea, a trait that the user would implement:

```rust
enum StorageKey {
    Permission(String),
    // Other persistent data, maybe resumable shards?
}

enum StorageValue {
    // ...
}

#[async_trait]
trait Storage {
    type Ok;
    type Err;
    async fn get(key: StorageKey) -> Result<Ok, Err>;
    async fn set(key: StorageKey, value: StorageValue) -> Result<Ok, Err>;
}
```

Maybe this could be some sort of extension crate?

### Notes
- Decl macro style definitions don't really fit into the style of Jumia
- Derive macros are a future goal
- Pure router style isn't very good
