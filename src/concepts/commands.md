# Commands

### Example

#### Concurrency

`&` and `|` - evaluate lhs, check it, then evaluate rhs
`&&` and `||` - evaluate lhs and rhs at the same time, then check

#### Axum-style extract arguments

Aren't they cool?

```rust
named_argument!(EchoText, "text", String);

async fn echo_command(ctx: extract::Context, text: extract::Arg<EchoText>) -> Result<(), BotError> {
    ctx.interaction.reply(ctx, text).await?;
    Ok(())
}

fn example() {
    Client::builder()
        .command(
            CommandBuilder::new()
                .name("echo")
                .option(
                    OptionBuilder::new()
                        .name("text")
                        .kind(OptionKind::String)
                )
                // ---
                .require(RequireGuild() && RequireCustom(123))
                // equivalent to
                .require(ConcurrentAnd(RequireGuild(), RequireCustom(123)))
                // ---
                .callback(echo_command)
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
- Maybe we should use "check" instead of "require", since it's more familiar wording
