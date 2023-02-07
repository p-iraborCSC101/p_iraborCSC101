fn main() {
	let mut sibling:Vec<String> = Vec::new();

	let mut input1 = String::new();
	println!("how many siblings do you have");
	std::io::stdin().read_line(&mut input1).expect("Failed to read input");
	let sib_num:i32 = input1.trim().parse().expect("Invalid input");

	let mut input2 = String::new();
	println!("");
}