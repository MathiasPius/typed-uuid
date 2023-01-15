# typed-uuid [![Latest Version]][crates.io] [![Docs]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/typed-uuid
[crates.io]: https://crates.io/crates/typed-uuid
[Docs]: https://docs.rs/typed-uuid/badge.svg
[docs.rs]: https://docs.rs/typed-uuid

<!-- cargo-rdme start -->

`Id` is a typed wrapper around a `uuid::Uuid`.

Use it to add type safety and prevent confusion between different kinds of Uuid.

## Example
Represent different types of Id to prevent mixups or invalid states. If describing
a unique resource's relationship to another, for example the `Role` a `User` has,
the relationship can be expressed as follows:
```rust
// Subtype the Id type to specify the version of the Id, instead
// of repeating yourself everywhere.
type Id<T> = typed_uuid::Id<T, typed_uuid::V4>;

struct Relation {
    user: Id<User>,
    role: Id<Role>,
}
```
`Id`s with different `T` parameter types are incompatible, and cannot be compared.

Attempting to assign an `Id<User>` to a variable of type `Id<Role>` is a compilation error.
```rust
let user = Id::<User>::new();
let role = Id::<Role>::new();

// Compilation fails here, can't compare Id<User> and Id<Role>
assert_eq!(user, role);
```

But `Id`s of the same type work:
```rust
let mut first = Id::<User>::new();
let second = Id::<User>::new();

first = second;
assert_eq!(first, second);
```

<!-- cargo-rdme end -->
