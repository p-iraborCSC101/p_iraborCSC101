fn main() {

	let mut numbers = [1,2,3,4];
	println!("original array = {:?}", numbers);

	let sliced_numbers = &mut numbers[1..4];
	println!("first slice = {:?}", sliced_numbers);

	sliced_numbers[2] = 8;

	println!("changed slice = {:?}", sliced_numbers);
}