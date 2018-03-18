// Add dependency text_io = "0.1.7"

//#[macro_use] extern crate text_io;

//use std::io;			// To make pause possible
//use std::io::prelude::*;	// To make pause possible
/*
// Function to test chords 
fn testonechord (targetchord: usize, targetsounds: &[usize], usedsounds: &[usize]) -> u8 {
	for x in 0..targetsounds.len() {
		if usedsounds.contains(&mutate(targetchord+&targetsounds[x], 12)) {} else {break};
		if x == targetsounds.len()-1 {println!("C Duuri!")};
	}		
	1	
}

// Mutates input so that when over number of sounds, starts from beginning
fn mutate(input: usize, limit: usize) -> usize {
	if input < limit {input} else {input - limit}
}
*/


// Function to test chords 
fn testonechord (targetchord: usize, targetsounds: &[usize], usedsounds: &[usize]) -> u8 {
	for x in 0..targetsounds.len() {
		if usedsounds.contains(&mutate(targetchord+&targetsounds[x], 12)) {} else {break};
		if x == targetsounds.len()-1 {println!("C Duuri!")};
	}		
	1	
}

fn main() {
    
	println!("Please enter some sounds separated with whitespace:");
	//let inputtext: String = read!("{}\n");  // For linux, for windows edit \n -> \r\n

	let inputtext = "C E G";	// manuaalinen, poista  A D B F C# D# F# G# Bb

	let inputnotes: Vec<&str> = inputtext.split(' ').collect();
	let allsounds = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "Bb", "B"];
	let mut allsounds_index = vec![];
	for i in 0..allsounds.len() {allsounds_index.push(i)}	
	let duuri = vec![0, 4, 7];	

	let mut inputnumbers = vec![];
	
	for x in 0..allsounds.len() {
		if testonechord(allsounds[x], &duuri, &inputnotes) == 1 {};
	}
	
	/*
	// Look for index number for each sound inside sounds group
	for x in 0..inputnotes.len() {	
		let index = allsounds.iter().position(|&r| r == inputnotes[x]).unwrap();		
		inputnumbers.push(index);
		}
	
	for x in 0..allsounds_index.len() {
		if testonechord(allsounds_index[x], &duuri, &inputnumbers) == 1 {};
	}
	*/

	

	println!("Loppu!");
	//println!("{:?}", allsounds_index);





//pause();
}

// ----------------------------------------------

/*
// Function for pause
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
*/
