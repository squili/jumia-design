# Commands

This is an example of an implementation of a command

```rust
// library code
trait CommandCallback {
    type Error: Into<JumiaError>;

    async fn call(&self, client: &Client, interaction: &Interaction, args: &CommandArgs) -> Result<(), Self::Error>;
}

impl<E, F, G> CommandCallback for F
where
    E: Into<JumiaError>,
    F: Fn(&Client, &Interaction, &CommandArgs) -> G,
    G: Future<Output = Result<(), E>>,
{
    type Error = E;

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
        .command(
            // there could probably be a better way of formatting this, but for now it's good enough
            CommandBuilder::new()
                .name("echo")
                .option(OptionBuilder::new()
                    .name("text")
                    .kind(OptionKind::String))
                .callback(guild_only(echo_command))
        );
}
```

### Notes
- Decl macro style definitions don't really fit into the style of Jumia
- Derive macros are a future goal
- Pure router style isn't very good
- Custom permission system using storage
- Arg parsing using system similar to serenity-slash-decode
