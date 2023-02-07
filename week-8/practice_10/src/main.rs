fn main() {
	let numbers = [1,2,3,4,5];
	println!("original array is : {:?}", numbers);

	let slice1 = &numbers[1..3];
	println!("2nd and 3rd elements are {:?}", slice1);

	let slice2 = &numbers[..3];
	println!("the starting elements are {:?}", slice2);

	let slice3 = &numbers[2..];
	println!("the ending elements are {:?}", slice3);

	let slice4 = &numbers[..];
	println!("{:?}", slice4);
}