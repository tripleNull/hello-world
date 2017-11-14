use std::{i32};
use std::io::stdin;

fn main() {

	let p1 = "You Guessed ";
	'outer: loop{
		let number: i32 = 10;
		println!("Pick a number");

		loop{
			let mut line = String::new();
			let input = stdin().read_line(&mut line);
			let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

			match guess {
				None => println!("Enter a number"),
				Some(n) if n == number => {
					println!("{}it!", p1);
					break 'outer;
				}
				Some(n) if n < number => {
					println!("{}too low", p1);
				}
				Some(n) if n > number => {
					println!("{}too high", p1);
				}
				Some(_) => println!("Error"),
			} 
		}
	}
}