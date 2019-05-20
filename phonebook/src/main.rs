/*
* Ege University ICI Programming Languages Project
* Rust based Phonebook - 1.0
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
	let book: &mut Vec<Record> = &mut Vec::new();

	loop {
		match menu() {
			1 => view(book),
			2 => search(book),
			3 => add(book),
			4 => delete(book),
			5 => exit(),
			_ => error(),
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
	println!("1: View\n2: Search\n3: Add\n4: Delete\n5: Exit");
	let input = read();
	let choice: i32 = input.trim().parse().expect("Enter correct choice.");
	if choice > 6 || choice < 1 {
		println!("Error, unable to decide choice number. Please retry.");
	}
	choice
}

fn view(book: &mut Vec<Record>) {
	println!("{}", Green.bold().paint("==View records=="));

	println!("List of all contacts:");
	for (i, f) in book.iter().enumerate() {
		println!("id: {} name: {} surname: {} home number: {} work number: {} address: {}", i, f.name, f.surname, f.home_number, f.work_number, f.address);
	}
}

fn search(book: &mut Vec<Record>) {
	println!("{}", Green.bold().paint("==Search record=="));
	println!("Search using (1) Phone number or (2) Name?");
	let mut input = read();
	let choice: i32 = input.trim().parse().expect("Enter correct choice");

	match choice {
		1 => {
			println!("Enter Home or Work number: ");
			input = read();
			for (i, f) in book.iter().enumerate() {
				if f.home_number == input || f.work_number == input {
					println!("id: {} name: {} surname: {} home number: {} work number: {} address: {}", i, f.name, f.surname, f.home_number, f.work_number, f.address);
				}
			}
		}

		2 => {
			println!("Enter Name: ");
			input = read();
			for (i, f) in book.iter().enumerate() {
				if f.name == input {
					println!("id: {} name: {} surname: {} home number: {} work number: {} address: {}", i, f.name, f.surname, f.home_number, f.work_number, f.address);
				}
			}
		}
		_ => {
			println!("Enter correct choice.");
		}
	}
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

fn delete(book: &mut Vec<Record>) {
	println!("{}", Green.bold().paint("==Delete record=="));
	println!("Enter Home or Work number: ");
	let input = read();
	let mut delete_list: Vec<usize> = [].to_vec();
	for (i, f) in book.iter().enumerate() {
		if f.home_number == input || f.work_number == input {
			println!("Match found: ");
			println!("id: {} name: {} surname: {} home number: {} work number: {} address: {}", i, f.name, f.surname, f.home_number, f.work_number, f.address);
			println!("Deleting entry...");
			delete_list.push(i);
		}
	}

	for i in delete_list {
		book.remove(i);
	}
}

fn exit() {
	println!("Thank you for using phonebook. See you later..");
	process::exit(0);
}

fn error() {
	println!("Error occured. Please try again..");
	process::exit(1);
}
