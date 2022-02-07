# Commands

This is an example of an implementation of a command

```rust
// library code
trait CommandCallback {
    type Error: Into<JumiaError>;
    
    async fn call(&self, ctx: &Context, interaction: &Interaction, args: &CommandArgs) -> Result<(), E>;
}

impl<E, F, G> CommandCallback<E> for F
where 
    E: Into<JumiaError>,
    F: Fn(&Client, &Interaction, &CommandArgs) -> G,
    G: Future<Output = Result<(), E>>,
{
    type Error = E;
    
    async fn call(&self, ctx: &Context, interaction: &Interaction, args: &CommandArgs) -> Result<(), E> {
        self(ctx, interaction, args)
    }
}

// either library or user code
trait GuildOnly<F: CommandCallback> {
    fn guild_only(callback: impl CommandCallback) -> F;
}

impl<E: Into<JumiaError>, C: CommandCallback<E>, F: CommandCallback> GuildOnly<F> for C
{
    fn guild_only(self) -> F {
        |ctx, interaction, args| async move {
            if interaction.guild_id.is_none() {
                interaction.reply(ctx, "Command must be run in guild").await?;
            } else {
                callback(ctx, interaction, args).await?;
            }
        }
    }
}

// user code
async fn echo_command(ctx: Context, interaction: Interaction, args: CommandArgs) -> Result<(), BotError> {
    interaction.reply(ctx, args.get_string("text")).await?;
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
                .callback(echo_command.guild_only())
        );
}
```

### Notes
- Decl macro style definitions don't really fit into the style of Jumia
- Derive macros are a future goal
- Pure router style isn't very good
- Custom permission system using storage
- Arg parsing using system similar to serenity-slash-decode
- Vicky suggested an alternative way of doing this:
  [d/serenity-rs Message Link](https://discord.com/channels/381880193251409931/381880193700069377/940070735349698600)
- Look more at how web frameworks do this
