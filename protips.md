## Essential tips I have found useful when writing Rust

https://rustbyexample.com/


Link to other file:
```rust
include!("src/doctest_helper.rs");
```
Create a function to which to refer (Remember! the last value in the function can be used as a return value.):
```rust
fn mutate(input: usize, limit: usize) -> usize {
	if input < limit {input} else {input - limit}
}
```
Create a loop
```rust
let mut counter = 0;
let mut hits = 0;
loop  {
		if scalenumbers.contains(&soundnumbers[counter]) {hits += 1} else {};		
		if counter == soundnumbers.len()-1 {break hits};
		counter += 1;
		};
```
