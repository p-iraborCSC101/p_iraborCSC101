use std::io;
use std::io::Read;

fn main() {
	println!("Please select a number: \n1. Administrator \n2.Project Manager \n3.Employee \n4.Customer \n5.Vendor");

	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let user:i32 = input1.trim().parse().expect("Failed to read number");

	match user {
		1 => {
			let mut file1 = std::fs::File::open("globacom_dbase.sql").unwrap();
	        let mut contents1 = String::new();
	        file1.read_to_string(&mut contents1).unwrap();
	        println!("{}", contents1);
		}

		2 => {
			let mut file2 = std::fs::File::open("project_tb.sql").unwrap();
	        let mut contents2 = String::new();
	        file2.read_to_string(&mut contents2).unwrap();
	        println!("{}", contents2);
		}

		3 => {
			let mut file3 = std::fs::File::open("staff_db.sql").unwrap();
	        let mut contents3 = String::new();
	        file3.read_to_string(&mut contents3).unwrap();
	        println!("{}", contents3);
		}

		4 => {
			let mut file4 = std::fs::File::open("customer_db.sql").unwrap();
	        let mut contents4 = String::new();
	        file4.read_to_string(&mut contents4).unwrap();
	        println!("{}", contents4);
		}

		5 => {
			let mut file5 = std::fs::File::open("dataplan_db.sql").unwrap();
	        let mut contents5 = String::new();
	        file5.read_to_string(&mut contents5).unwrap();
	        println!("{}", contents5);
		}

		_=> println!("Invalid selection"),

	}

}