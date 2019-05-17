/*
* Ege University ICI Programming Languages Project
* Phonebook - 1.0
* Dogan C. Karatas
* May 2019 - GNU GPL v3 Licensed..
*/

use std::io;
use ansi_term::Colour::*;
use std::process;

struct Record {
	name: String,
	surname: String,
	home_number: String,
	work_number: String,
	address: String,
}

fn main() {
	println!("{}", Blue.bold().paint("Phonebook"));
	let mut book: &mut Vec<Record> = &mut Vec::new();

	/*
	let bob: Record = Record { name: String::from("Bob"), surname: String::from("Yannes"), home_number: String::from("123123"), work_number: String::from("232452"), address: String::from("address") };
	book.push(bob);
	
	for f in book.iter() {
		println!("{} {} {} {} {}", f.name, f.surname, f.home_number, f.work_number, f.address);
	}
	*/

	loop {
		match menu() {
			1 => view(),
			2 => search(),
			3 => add(book),
			4 => update(),
			5 => delete(),
			6 => exit(),
			_ => error(),
		}

	for f in book.iter() {
		println!("{} {} {} {} {}", f.name, f.surname, f.home_number, f.work_number, f.address);
	}
	}
}

fn read() -> String {
	let stdin = io::stdin();
	let input = &mut String::new();
	input.clear();
	stdin.read_line(input).expect("Couldn't read from std::io::stdin()");
	input.trim().to_string()
}

fn menu() -> i32 {
	println!("1: View\n2: Search\n3: Add\n4: Update\n5: Delete\n6: Exit");
	let input = read();
	let choice: i32 = input.trim().parse().expect("Enter correct choice.");
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

fn add(book: &mut Vec<Record>) {
	println!("{}", Green.bold().paint("==Add record=="));
	println!("Enter name: ");
	let name = read();
	println!("Enter surname: ");
	let surname = read();
	println!("Enter home number: ");
	let home_number = read();
	println!("Enter work number: ");
	let work_number = read();
	println!("Enter address: ");
	let address = read();
	
	let person = Record {name: name, surname: surname, home_number: home_number, work_number: work_number, address: address};
	book.push(person);
	println!("Person saved into phonebook.");
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
