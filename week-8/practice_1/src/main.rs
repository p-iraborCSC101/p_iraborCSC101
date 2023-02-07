fn main() {

	let v:Vec<i64> = Vec::new(10,20,30);

	println!("\nThe length of Vec::new is {}",v.len());

	let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

	println!("\nThe length of vec macro is: { }",v.len());
}