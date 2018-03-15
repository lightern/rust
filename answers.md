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

Ex. 3
```rust
fn something() -> Result<i32, std::num::ParseIntError> {
	let x:i32 = "3".parse().unwrap();
	Ok(x * 4)
}


fn main() {
    match something() {
        Ok(t) => println!("You win!"),
        Err(e) => println!("Oh no something went wrong: {}", e),
    }
}
```
