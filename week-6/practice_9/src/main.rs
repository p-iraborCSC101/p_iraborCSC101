fn main() {
	let city_arr:[&str;5] = ["abuja","portharcourt","maiduguri","kano","lagos"];
	println!("array is {:?}",city_arr);
	println!("array size is {}",city_arr.len());

	for index in city_arr.iter() {
		println!("city index is located in {}",index );
	}
}