fn main() {
	let p:i64 = 210_000;
	let r:i64 = 5;
	let t:i64 = 3;

	
	let interest = (p * r * t)/100;
	println!("Interest is {}", interest);

	let value = p - interest;
	println!("New value is {}", value);
}