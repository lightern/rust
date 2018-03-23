## Mitä kaikkea voi tehdä?

https://rustbyexample.com/


Luoda funktion, johon voi viitata (Muista! funktion viimeistä arvoa voidaan käyttää paluuarvona): 

```rust
fn mutate(input: usize, limit: usize) -> usize {
	if input < limit {input} else {input - limit}
}
```
Luoda loopin: 
```rust
let mut counter = 0;
let mut hits = 0;
loop  {
		if scalenumbers.contains(&soundnumbers[counter]) {hits += 1} else {};		
		if counter == soundnumbers.len()-1 {break hits};
		counter += 1;
		};
```
