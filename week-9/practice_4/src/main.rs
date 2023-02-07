use std::fs::OpenOptions;
use std::io::Write;

fn main() {

	let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
	file.write_all("\nhello class".as_bytes()).expect("write failed");
	file.write_all("\nthis is the appendage".as_bytes()).expect("write failed");

	println!("append success");

}