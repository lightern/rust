## Essential tips I have found useful when writing Rust

https://rustbyexample.com/

http://words.steveklabnik.com/pointers-in-rust-a-guide

Create a basic function to which to refer (Remember! the last value in the function can be used as a return value.):
```rust
fn mutate(input: usize, limit: usize) -> usize {
	if input < limit {input} else {input - limit}
}
```
Link to other file:
```rust
include!("src/doctest_helper.rs");
```
Create a loop that modifies some value (doesn't work with for loops in Rust):
```rust
let mut counter = 0;
let mut hits = 0;
loop  {
		if somevector.contains(&somevector[counter]) {hits += 1} else {};		
		if counter == somevector.len()-1 {break hits};
		counter += 1;
		};
```
