use std::io;

fn main() {
	let mut input1 = String::new();
	println!("Please input the food type");
	io::stdin().read_line(&mut input1).expect("failed to read input");
	let food_type:&str = input1.trim().parse().expect("failed to read input");

	let mut input2 = String::new();
	println!("Please input the quantity of food");
	io::stdin().read_line(&mut input2).expect("failed to read input");
	let quantity:&str = input2.trim().parse().expect("failed to read input");


	
}