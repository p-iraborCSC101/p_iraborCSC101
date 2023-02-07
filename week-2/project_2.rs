fn main() {
	let t = 450_000;
	let m = 1_500_000;
	let h = 750_000;
	let d = 2_850_000;
	let a = 250_000;
	println!("Toshiba is {} \nMac is {} \nHp is {} \nDell is {} \nAcer is {}",t,m,h,d,a);

	let s = t + m + h + d + a;
	println!("Sum is {}",s);

	let n = 2 + 1 + 3 + 3 + 1;
	let v = s / n;
	println!("Average is {} ",v); 
}