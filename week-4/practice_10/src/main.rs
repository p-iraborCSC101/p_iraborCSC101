fn main() {
    let a = 20;
    let b = 30;

    if (a > 10) && (b > 10) {
    	println!("true");
    }
    let c = 0;
    let d = 0;

    if (c > 10) || ( d > 10) {
    	println!("false");
    }
    let is_elder = false;

    if !is_elder {
    	println!("Not Elder");
    }
}
