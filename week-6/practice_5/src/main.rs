fn main(){
	let num:i32 = 2;
	mutate_num_to_five(num);
	println!("The value of no is:{}",num);
}

fn mutate_num_to_five(mut param_num: i32) {
	param_num = param_num * 2;
	println!("param_num value is :{}",param_num);
}