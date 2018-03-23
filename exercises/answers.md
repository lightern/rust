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

Ex. 4
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

Ex. 5
```rust
enum Reaction<'a> {
    Sad(&'a str),
    Happy(&'a str),
}

fn express(sentiment: Reaction) {
    match sentiment {
        Reaction::Sad(s) => println!(":( {}", s),
        Reaction::Happy(s) => println!(":) {}", s),
    }
}

fn main () {
    let x = Reaction::Happy("It's a great day for Rust!");
    express(x);
    //express(x);
    let y = Reaction::Sad("This code doesn't compile yet.");
    express(y);
}
```
