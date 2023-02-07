fn main() {

	let details:(&str, i32, f64) = ("paula", 16, 5.6);
	print(details); 
}

fn print(x:(&str, i32, f64)) {
	let (name, age, height) = x;
	println!("my name is {} \nmy age is {} \nmy height is {}", name,age,height);
}