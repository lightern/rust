#[macro_use] extern crate text_io;

struct hits { count: 

fn main() {

let inputtext: String = read!("{}\n");

let notes: Vec<&str> = inputtext.split(' ').collect();
//assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let sounds = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
let scalenumbers = vec![0, 2, 4, 5, 7, 9, 11];
let scales = vec!["Major", "Dorian", "Phrygian", "Lydian", "Mixolydian", "Minor", "Locrian"];


let mut test = 0;

for x in 0..notes.len() {
// haetaan monesko kokeiltava nuotti on
let index = sounds.iter().position(|&r| r == notes[x]).unwrap();
// tsekataan onko kyseinen numero skaalanumerossa
let test = if scalenumbers.contains(&index) {1} else {0};
}
println!("{}", test);


//println!("{}", index);

//println!("{}", notes[2]);
//println!("{} {} {} {} {} {} {}", sounds[0], sounds[2], sounds[4], sounds[5], sounds[7], sounds[9], sounds[11]);


//println!("{}", nuotti);
}
