# Commands

### Example

#### Concurrency

`&` and `|` - evaluate lhs, check it, then evaluate rhs
`&&` and `||` - evaluate lhs and rhs at the same time, then check

#### Axum-style extract arguments

Aren't they cool? Sadly, adt_const_params is complicated, so we need a macro for now.

```rust
// until adt_const_params is stabilized, we have to do this
// it will expand to a struct that implements a trait specifying a const and a type
// this could also be made into a proc macro, but i don't know how to write those
// if someone who knows proc macros wants to help out, shoot me a message!
named_argument!(EchoText, "text", String);

async fn echo_command(ctx: Context, Arg(text): Arg<EchoText>) -> Result<(), BotError> {
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
- Arg parsing using system similar to serenity-slash-decode
- Vicky suggested an alternative way of doing this:
  [d/serenity-rs Message Link](https://discord.com/channels/381880193251409931/381880193700069377/940070735349698600)
- Maybe we should use "check" instead of "require", since it's more familiar wording
