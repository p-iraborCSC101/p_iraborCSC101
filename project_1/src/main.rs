use std::io::Write;
use std::fs::File;

fn main() {

let lager = "The drinks that fall under lager are; 33 Export, Desperados, Goldberg, Gulder, Heineken, Star";
let stout = "\nThe drinks that fall under stout are; Legend, Turbo King, Williams";
let non_alcoholic = "\nThe drinks that fall under non-alcholic are;  Maltina, Amstel Malta, Malta Gold, Fayrouz";

let mut file = std::fs::File::create("Nigerian Breweries Drink Categories.txt").expect("create failed");
file.write_all(lager.as_bytes()).expect("write failed");
file.write_all(stout.as_bytes()).expect("write failed");
file.write_all(non_alcoholic.as_bytes()).expect("write failed");

println!("file create success");
 }



 