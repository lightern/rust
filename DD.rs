// Watch a great video tutorial for this here: https://www.youtube.com/watch?v=grU-4u0Okto

// 1) Let's create structs to represent classes

struct Dwarf {
name: String
}
struct Elf {
name: String
}
struct HalfOrc {
name: String
}
struct Human {
name: String
}
struct HalfElf {
name: String
}


// 10) Trait Objektit voivat muista traiteista poiketen sisältää tekemisen lisäksi dataa. Trait Objektit sisältävät pointterin heapissa olevaan dataan. Trait Objekteihin ei kuitenkaan voi lisätä dataa. Aloitetaan luomalla taikoja varten structeja, joihin tekemistä voidaan lisätä. Kaikkia voi "castata" (vaikkakin castaamisen tapa voikin erota).
struct Cantrip {
}
struct Transmutation {
}
struct Enchantment {
}
struct Necromancy {
}

//13 Luodaan Spellbook jossa pitää taikoja. Luodaan siis structi.
struct Spellbook {
pub spells: Vec<Box<Cast>>,}	// spells -field on vectori (tapa groupata objekteja, jotka ovat tiettyä tyyppiä). Tyyppi, jota grouppaamme taioille on Box(Box<T>). Box taas on pointer, joka osoittaa dataan heapissa (ks. kohta 10). Nyt sanotaan, että voidaan ottaa vastaan mikä tahansa tyyppi, joka implementoi Cast Traitin. Tänne siis otetaan vastaan yllä olevat structit, jotka ovat taikaluokkia

// 3) Let's create traits (features/attributes) and in those traits define one function (bonus)
pub trait Constitution {
fn constitution_bonus(&self) -> u8 {0} // We don't define how the constitution_bonus is used, we just expect it to be used somehow. Also we define default to be 0.
}
// 6) Let's create a language (trait), that only Elf's talk: Elvish (we also need a function for that, since it's not an attribute but doing!)
pub trait Elvish {
}
// 11) Luodaan Cast trait
pub trait Cast {
fn cast(&self);			// jokaisen castaajan pitää määrittää behaviour cast functiolle
}

// 4) Now let's implement that trait to class struct to override class specific parts if needed and along with implementing we want to define how the constitution_bonus works
impl Constitution for Dwarf {
fn constitution_bonus(&self) -> u8 {2}
}
impl Constitution for HalfOrc {
fn constitution_bonus(&self) -> u8 {1}
}
impl Constitution for Elf {}		// We don't need to define the bonus and they will inherit the default 0
impl Constitution for Human {}		// We don't need to define the bonus and they will inherit the default 0

// 7) Let's implement Elvish language only for Elfs:
impl Elvish for Elf {}
impl Elvish for HalfElf {}

// 12) Implementoidaan Cast Trait Cantrip structille (taialle)
impl Cast for Cantrip {			// Let's implement Cast for struct Cantrip
fn cast(&self) { 			// Details of casting a Cantrip Spell
println!("Cantrip casted!");
}
}
impl Cast for Transmutation {
fn cast(&self) { 			// Details of casting a Transmutation Spell
println!("Transmutation casted!");
}
}
impl Cast for Enchantment {
fn cast(&self) { 			// Details of casting a Transmutation Spell
println!("Enchantment casted!");
}
}
impl Cast for Necromancy {
fn cast(&self) { 			// Details of casting a Transmutation Spell
println!("Necromancy casted!");
}
}

// 14) Let's define some behaviour for Spellbook struct in implementation block. Spellbook
impl Spellbook {
pub fn run(&self) {
for spell in self.spells.iter() {
spell.cast();
}
}
}

// 8) Let's create a function for speaking Elvish:
pub fn speak_elvish<T: Elvish>(character: T) -> String {	// Accept argument (character: T)as generic type T, so it doesn't need to be specific type (it can be even a struct). BUT! We only accept those types T, that implements Elvish traits (<T: Elvish>), ie. Elf and HalfElf structs!. You can have even multiple traits that is needed to do something.
String::from("yes")
}



fn main() {

// 2) Let's create charaters as instances of structs
let my_dwarf = Dwarf {
name: String::from("NellDwarf")
};
let my_half_orc = HalfOrc {
name: String::from("NellOrc")
};
let my_elf = Elf {
name: String::from("NellElf")
};
let my_human = Human {
name: String::from("Nell")
};
let my_halfelf = HalfElf {
name: String::from("NellHalfElf")
};


// 15 Implementoidaan Spellbook implementaation käyttö
let spell_book = Spellbook {
spells: vec![
Box::new(Cantrip{}),
Box::new(Transmutation{}),
Box::new(Enchantment{}),
Box::new(Necromancy{}),
],
};

// 5) käytetään dwarffin constitution bonusta
println!("Dwarfin constitution: {}", my_dwarf.constitution_bonus());
println!("HalfOrcin constitution: {}", my_half_orc.constitution_bonus());
println!("HalfOrcin constitution: {}", my_elf.constitution_bonus());
println!("HalfOrcin constitution: {}", my_human.constitution_bonus());

// 9) Laitetaan Elf luokan instanssi (ilmentymä) my_elf (nimeltä NellElf), puhumaan Elvishtä:
println!("Elf: {}", speak_elvish(my_elf));
println!("HalfElf: {}", speak_elvish(my_halfelf));

// 16) Ajetaan koko spellbook
spell_book.run();
}
