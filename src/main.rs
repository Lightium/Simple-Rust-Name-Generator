extern crate rand;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use rand::distributions::{Sample, Range};

fn main() {
	let mut out = String::new();
    
	let mut f = File::open("first").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong");

	let mut f2 = File::open("second").expect("File not found");
    let mut contents2 = String::new();
    f2.read_to_string(&mut contents2).expect("Something went wrong");

	let mut f3 = File::open("third").expect("File not found");
    let mut contents3 = String::new();
    f3.read_to_string(&mut contents3).expect("Something went wrong");
	
	let first : Vec<&str> = contents.split(";").collect();
	let second : Vec<&str> = contents2.split(";").collect();
	let third : Vec<&str> = contents2.split(";").collect();
	
	let mut between = Range::new(1, 4);
	let mut rng = rand::thread_rng();

	let a = between.sample(&mut rng);
	let b = between.sample(&mut rng);
	let c = between.sample(&mut rng);

	match a {
		1 => {
			out += rand::thread_rng().choose(&first).unwrap();
		},
		2 => {
			out += rand::thread_rng().choose(&second).unwrap();
		},
		3 => {
			out += rand::thread_rng().choose(&third).unwrap();
		},
		_ => {
			println!("An error occured");
		}
	}
	
	match b {
		1 => {
			out += rand::thread_rng().choose(&first).unwrap();
		},
		2 => {
			out += rand::thread_rng().choose(&second).unwrap();
		},
		3 => {
			out += rand::thread_rng().choose(&third).unwrap();
		},
		_ => {
			println!("An error occured");
		}
	}
	
	match c {
		1 => {
			out += rand::thread_rng().choose(&first).unwrap();
		},
		2 => {
			out += rand::thread_rng().choose(&second).unwrap();
		},
		3 => {
			out += rand::thread_rng().choose(&third).unwrap();
		},
		_ => {
			println!("An error occured");
		}
	}
	println!("{:?}", out);
}
