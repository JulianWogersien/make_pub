# make_pub
This rust macro makes all fields in the struct its used on public. Useful for example when you have a large struct that stores configuration

To use this simply use this as a macro on a struct like so
```rust
#[make_pub]
#[derive(Debug)]
struct Conf {
    field1: i32,
    field2: f32,
}
```

Then you may check with cargo expand that the previously private fields have become public
