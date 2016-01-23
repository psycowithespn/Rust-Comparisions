extern crate rand;

use rand::Rng;

fn main() {
	let mut rng = rand::thread_rng();
	println!("Hello World {}", rng.gen::<i32>());
}