/* 
* Ege University ICI Programming Languages Project
* Phonebook - 1.0
* Dogan C. Karatas
* May 2019 - GNU GPL v3 Licensed..
*/

use std::io;
use ansi_term::Colour::*;
use std::process;

fn main() {
	println!("{}", Blue.bold().paint("Phonebook"));
	match menu() {
		1 => view(),
		2 => search(),
		3 => add(),
		4 => update(),
		5 => delete(),
		6 => exit(),
		_ => error(),
	}
}


fn menu() -> i32 {
	println!("1: View\n2: Search\n3: Add\n4: Update\n5: Delete\n6: Exit");
	
	let stdin = io::stdin();
	let input_buffer = &mut String::new();
	input_buffer.clear();
	stdin.read_line(input_buffer).expect("Couldn't read from std::io::stdin()");
	let choice: i32 = input_buffer.trim().parse().expect("Enter correct choice.");
	if choice > 6 || choice < 1 {
		println!("Error, unable to decide choice number. Please retry.");
	}
	choice
}

fn view() {
	println!("{}", Green.bold().paint("==View records=="));
}

fn search() {
	println!("{}", Green.bold().paint("==Search record=="));
}

fn add() {
	println!("{}", Green.bold().paint("==Add record=="));
}

fn update() {
	println!("{}", Green.bold().paint("==Update record=="));
}

fn delete() {
	println!("{}", Green.bold().paint("==Delete record=="));
}

fn exit() {
	println!("Thank you for using phonebook. See you later..");
	process::exit(0);
}

fn error() {
	println!("Error occured. Please try again..");
	process::exit(1);
}
