use std::io;

fn GeoPo_Merger() {

let name_arr::[&str;5] = [Aigbogun Daudu, Murtala Afeez Bendu, Okorocha Calistus Ogbona, Adeale Jimoh Akanbi, Osazuwa Faith Etieye];
	println!("Names of Commisioner");
	println!("Names of Commisioner are {:?}", name_arr);
	println!("Names of Commisioner size is: {}",name_arr.len());
	let ministry_arr:[&str;5] = [Internal Affairs, Justice, Defense, Power and Steel, Petroleum];
	println!("Ministry");
	println!("Ministry is {:?}",ministry_arr4);
	println!("Ministry size is: {}",ministry_arr.len());
	let zone_arr:[&str;5] = [South West, North East, South South, South West, South East];

	for index in 0..5 {
		println!("{} is in {} from {} zone",index,name_arr,ministry_arr,zone_arr[index]);
	}
}
fn Pub_Service() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();

	println!("Enter name: ");
	io::stdin().read_line(&mut input1).expect("Not a valid string");

	println!("Enter your position");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let mut position = input2.trim().parse().expect("Not a valid position");

	if position = intern {
		println!("How long have you worked");
		io::stdin().read_line(&mut input3).expect("Not a valid string");
	let mut position = input3.trim().parse().expect("Not a valid position");

	 if experience >= 1 && experience <= 2 {
	 	println!("Congratulations you are promoted to Adminstrator");
	 } 

	}
}

fn main() {
	GeoPo_Merger();
}