//develop a program that reads the personal details of the students from  an array or vector,//
// then displays the details and save into a file in the following format//
//student name, matric number, department, level//
use std::io::Write;

fn main() {
let student_name:[&str;5] = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
let matric_number:[&str;5] = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
let dept:[&str;5] = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
let lvl:[&str;5] = ["300", "100", "200","200", "100"];

for x in 0..lvl.len() {
	println!("{}'s matric number is {} and the student is in {} {} level", 
		student_name[x],matric_number[x],dept[x],lvl[x]);
	
}

let mut file = std::fs::File::create("PAU SMIS").expect("create failed");
file.write_all("Welcome to PAU SMIS".as_bytes()).expect("write failed");
file.write_all("\nx".as_bytes()).expect("write failed");

println!("create success");
}





