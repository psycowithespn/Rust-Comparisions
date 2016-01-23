use std::cmp::Ordering;

fn main() {
	
	let number_one = 20;
	let number_two = 30;

	match number_one.cmp(&number_two) {
		Ordering::Greater => println!("Number 1 is bigger"),
		Ordering::Equal => println!("The numbers are equal"),
		Ordering::Less =>  println!("Number 2 is bigger")
	}
}