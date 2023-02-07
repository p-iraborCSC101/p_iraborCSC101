fn main() {
	let b:(i32, f64, bool) = (12, 5.6, false);
	print(b);
}

fn print(x:(i32, f64, bool)) {
	println!("{:?}", x);
}