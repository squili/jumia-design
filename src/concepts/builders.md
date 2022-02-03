# Builders

Most things in Jumia are created using builders. This is especially the case for actions.

### Conversion

In order to keep the builders ergonomic, many converter traits are utilized. For example, these are all valid usages of
the `CreateOverwrite` builder, through the `Into<RoleId>` trait.

```rust
fn example(some_role: Role, an_id: RoleId) {
    CreateOverwrite::builder()
        .role(some_role)
        .allow(Permissions::VIEW_CHANNEL);

    CreateOverwrite::builder()
        .role(an_id)
        .allow(Permissions::VIEW_CHANNEL);

    CreateOverwrite::builder()
        .role(123)
        .allow(Permissions::VIEW_CHANNEL);
}
```

### Todo
- Look into how rustc optimizes builders
