fn main() {
	let name = vec!["mary","sam","sally","greg","ade","mark","june","ife"];
	let age = vec![16,17,19,22,20,21,18,23];

	print!("\nage allocation\n");

	for a in 0..name.len() {
		print!("{} is {} years old\n", name[a],age[a]);
	}
}