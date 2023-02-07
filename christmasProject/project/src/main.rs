use std::io::Write;
use std::io;

fn code_7() {

	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("invalid input");
	let mut file:i32 = input1.trim().parse().expect("invalid input");

	match file 
	{
		1 => 
		{
		let cons = "Services offered : Analytics consulting services 
		                              \nCustomer experience 
		                              \nCybersecurity, strategy, risk, compliance and resilience 
		                              \nDigital transformation 
		                              \nRisk consulting services 
		                              \nSupply chain and operations 
		                              \nTechnology transformation";

		let mut new_file = std::fs::File::create("aigbona_juliet.txt").expect("create failed");
		new_file.write_all("Name: Aigbona Juliet\n".as_bytes()).expect("write failed");
		new_file.write_all("Department: Consulting\n".as_bytes()).expect("write failed");
		new_file.write_all(cons.as_bytes()).expect("write failed");
	    }
	 
	 2 => {
		let assurance = {"Services offered : Audit services 
		                                   \nClimate change and sustainabilty services 
		                                   \nFinancial accounting advisory services 
		                                   \nForensic and integrity services 
		                                   \nPrivate  client audit experience 
		                                   \nAccounting Link 
		                                   \nAssurance"};

		let mut new_file2 = std::fs::File::create("akpevwe_iloka.txt").expect("create failed");
		new_file2.write_all("Name: Akpevwe Iloka\n".as_bytes()).expect("write failed");
		new_file2.write_all("Department: Assurance\n".as_bytes()).expect("write failed");
		new_file2.write_all(assurance.as_bytes()).expect("write failed");
	}
	_ => println!("Invalid selection"),
	}
		
}



	



fn code_8() {

	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("invalid input");
	let mut file:i32 = input2.trim().parse().expect("invalid input");
	
	match file 
	{
		3 => {
		let tax = {"Services offered : Tax planning 
		                             \nTax function operations
		                             \nTax policy and controversy 
		                             \nGlobal trade
		                             \nTax accounting
		                             \nTax compliance
		                             \nTransaction tax"};

		let mut new_file3 = std::fs::File::create("adamu_sagamu.txt").expect("create failed");
		new_file3.write_all("Name: Adamu Sagumu\n".as_bytes()).expect("write failed");
		new_file3.write_all("Department: Tax\n".as_bytes()).expect("write failed");
		new_file3.write_all(tax.as_bytes()).expect("write failed");
	}
	 4 => {
		let paw = {"Services offered :  Change management and experience 
		                              \nHR transformation 
		                              \nIntegrated workforce mobility 
		                              \nLearning and development consulting 
		                              \nRecognition and reward advisory 
		                              \nWorkforce analytics 
		                              \nPeople and workforce"};

		let mut new_file4 = std::fs::File::create("gbenga_daniels.txt").expect("create failed");
		new_file4.write_all("Name: Gbenga Daniels\n".as_bytes()).expect("write failed");
		new_file4.write_all("Department: People and Workforce\n".as_bytes()).expect("write failed");
		new_file4.write_all(paw.as_bytes()).expect("write failed");
	}
	_ => println!("Invalid selection"),
}

}


fn code_9() {

	let mut input3 = String::new();
	io::stdin().read_line(&mut input3).expect("invalid input");
	let mut file:i32 = input3.trim().parse().expect("invalid input");

	match file {
		5 => {
		let strat = {"Services offered : Strategy consulting 
		                               \nCoorporate growth and strategy 
		                               \nTransaction strategy and execution 
		                               \nRestructuring and turnaround strategy 
		                               \nIndustry strategy 
		                               \nDigital business building 
		                               \nCommercial strategy"};

		let mut new_file5 = std::fs::File::create("ehis_ero.txt").expect("create failed");
		new_file5.write_all("Name: Ehis Ero\n".as_bytes()).expect("write failed");
		new_file5.write_all("Department: Strategy\n".as_bytes()).expect("write failed");
		new_file5.write_all(strat.as_bytes()).expect("write failed");
	}
	 6 => {
		let trans = {"Services offered : Commercial strategy 
		                               \nDivestments and carve-outs 
		                               \nSustainability and ESG Services 
		                               \nM&A advisory  
		                               \nM&A integration 
		                               \nM&A technology and tools 
		                               \nM&A advanced analytics"};

		let mut new_file6 = std::fs::File::create("maria_akinsola.txt").expect("create failed");
		new_file6.write_all("Name: Maria Akinsola\n".as_bytes()).expect("write failed");
		new_file6.write_all("Department: Transactions and Corporate Finance\n".as_bytes()).expect("write failed");
		new_file6.write_all(trans.as_bytes()).expect("write failed");
	}
	_ => println!("Invalid selection"),
}

}



fn main() {
	
		println!("Select a staff to create a file: \n1. Aigbona Juliet 
			\n2.Akpevwe Iloka 
			\n3.Adamu Sagumu 
			\n4.Gbenga Daniels 
			\n5.Ehis Ero 
			\n6.Maria Akinsola");



	code_7();
	code_8();
	code_9();
	println!("success");
}

