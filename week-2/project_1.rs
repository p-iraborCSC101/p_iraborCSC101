fn main() {

	let p:i64 = 520_000_000;
	let t:i64 = 5;
	let r:i64 = 1;

	println!("The principal, rate and time are {}, {} and {}",p,r,t);

	let a = p * (1 + (r / 100)) * t;
	println!("The amount is {} ",a);

	let ci = a - p;
	println!("Compound Interest is {}", ci);


}