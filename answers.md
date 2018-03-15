## carols10cents excercises

Ex. 2
```rust
fn something() -> String {"hi".to_string()}

fn main() {
    println!("{}", something());
}
```

Ex. 3
```rust
struct Foo {
    capacity: i32,
}

fn main() {
    println!("{:?}", Foo {capacity: 3}.capacity);
}
```
