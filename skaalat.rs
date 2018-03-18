#[macro_use] extern crate text_io;

//use std::io;				// In case I want to use pause instead of a loop
//use std::io::prelude::*;	// In case I want to use pause instead of a loop
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

	let notes: Vec<&str> = inputtext.split(' ').collect();

	let sounds = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "Bb", "B"];
	let scalenumbers = vec![0, 2, 4, 5, 7, 9, 11];
	let scales = vec!["Major", "Dorian", "Phrygian", "Lydian", "Mixolydian", "Minor", "Locrian"];
	let mut soundnumbers = vec![];
	let mut hitvec = vec![];

	// Calculate how many times every sound listed can be 
	for x in 0..12 {
	
		// Look for index number for each sound inside sounds group
		for y in 0..notes.len() {	
			let index = sounds.iter().position(|&r| r == notes[y]).unwrap() + 12 - x;		
			let modifiedindex = mutate(index, 12);
			soundnumbers.push(modifiedindex);
			}
	
		// Look how many times each index number can be found from restricting group
		let mut counter = 0;
		let mut hits = 0;
		loop  {
				if scalenumbers.contains(&soundnumbers[counter]) {hits += 1} else {};		
				if counter == soundnumbers.len()-1 {break hits};
				counter += 1;
				};
		hitvec.push(hits);
		soundnumbers.truncate(0);
		};

	//soundnumbers.push(soundnumbers[0].unwrap());

	for z in 0..hitvec.len() {
		println!("{} common: {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}", hitvec[mutate(z, 12)], sounds[mutate(z, 12)], scales[0], sounds[mutate(z+2, 12)], scales[1], sounds[mutate(z+4, 12)], scales[2], sounds[mutate(z+5, 12)], scales[3], sounds[mutate(z+7, 12)], scales[4], sounds[mutate(z+9, 12)], scales[5], sounds[mutate(z+11, 12)], scales[6]);
		}
//pause();
}
