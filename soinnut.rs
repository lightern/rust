// Add dependency text_io = "0.1.7"

#[macro_use] extern crate text_io;

//use std::io;			// If you want to remove loop and make it pause in the end instead
//use std::io::prelude::*;	// If you want to remove loop and make it pause in the end instead

// Function to test chords 
fn testonechord (targetchord: usize, targetsounds: &[usize], version: &'static str, usedsounds: &[usize]) -> u8 {
	let allsounds = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "Bb", "B"];
	for x in 0..targetsounds.len() {
		if usedsounds.contains(&mutate(targetchord+&targetsounds[x], 12)) {} else {break};
		if x == targetsounds.len()-1 {println!("{}{}", allsounds[targetchord], version)};
	}		
	1	// Added return value in case I want to do something with this in future
}

// Mutates input so that when over number of sounds, starts from beginning
fn mutate(input: usize, limit: usize) -> usize {
	if input < limit {input} else {input - limit}
}

fn main () {
	loop {
		testi();
	}
}

fn testi() {
    let inputtext = loop {
	println!("Please enter some sounds separated with whitespace (something else to exit):");
	let inputtext: String = read!("{}\r\n");  // For linux, for windows edit \n -> \r\n
	if inputtext.len() == 0 {} else {break inputtext;}
	};

	let inputnotes: Vec<&str> = inputtext.split(' ').collect();
	let allsounds = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "Bb", "B"];
	let mut allsounds_index = vec![];
	for i in 0..allsounds.len() {allsounds_index.push(i)}	
	let duuri = vec![0, 4, 7];	
	let molli = vec![0, 3, 7];	
	let _7 = vec![0, 4, 7, 10];
	let maj7 = vec![0, 4, 7, 11];
	let m7 = vec![0, 3, 7, 10];
	let _6 = vec![0, 3, 7, 9];
	
	let mut inputnumbers = vec![];
	
	// Look for index number for each sound inside sounds group
	for x in 0..inputnotes.len() {	
		let index = allsounds.iter().position(|&r| r == inputnotes[x]).unwrap();		
		inputnumbers.push(index);
		}
	
	for x in 0..allsounds_index.len() {
		if testonechord(allsounds_index[x], &duuri, "      (1 3 5)", &inputnumbers) == 1 {};
	}
	for x in 0..allsounds_index.len() {
		if testonechord(allsounds_index[x], &molli, "m     (1 b3 5)", &inputnumbers) == 1 {};
	}
	for x in 0..allsounds_index.len() {
		if testonechord(allsounds_index[x], &_7, "7     (1 3 5 b7)", &inputnumbers) == 1 {};
	}
	for x in 0..allsounds_index.len() {
		if testonechord(allsounds_index[x], &maj7, "maj7  (1 3 5 7)", &inputnumbers) == 1 {};
	}
	for x in 0..allsounds_index.len() {
		if testonechord(allsounds_index[x], &m7, "m7    (1 b3 5 b7)", &inputnumbers) == 1 {};
	}
	for x in 0..allsounds_index.len() {
		if testonechord(allsounds_index[x], &_6, "6     (1 b3 5 6c)", &inputnumbers) == 1 {};
	}
	
//pause();
}

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
