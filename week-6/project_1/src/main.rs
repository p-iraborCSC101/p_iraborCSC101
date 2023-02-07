use std::io;

fn main() {

	println!("\nWelcome, Please select an equation  to calculate: 
        \n1. Area of trapezium
	    \n2. For area of rhombus
	    \n3.For area of parallelogram
	    \n4. For area of cube
	    \n5. For volume of cylinder");

	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let equation:u32 = input1.trim().parse().expect("Failed to read number");

	match equation {
        1 => {

        println!("\nenter height");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let height:f32 = input2.trim().parse().expect("Failed to read input");

        println!("enter base1");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let base1:f32 = input3.trim().parse().expect("Failed to read input");

        println!("enter base2");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let base2:f32 = input4.trim().parse().expect("Failed to read input");

        area_of_trapezium(height, base1, base2);
        }
        
        2 => {
        println!("\nenter diagonal 1");
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let diagonal1:f32 = input5.trim().parse().expect("Failed to read input");

        println!("enter diagonal 2");
        let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let diagonal2:f32 = input6.trim().parse().expect("Failed to read input");

        area_of_rhombus(diagonal1, diagonal2);
        }

        3 => {
        println!("\nbase");
        let mut input7 = String::new();
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let base:f32 = input7.trim().parse().expect("Failed to read input");

        println!("enter altitude");
        let mut input8 = String::new();
        io::stdin().read_line(&mut input8).expect("Failed to read input");
        let altitude:f32 = input8.trim().parse().expect("Failed to read input");

        area_of_parallelogram(base, altitude);    

        }
        4 => {
            println!("\nenter length of side");
            let mut input9 = String::new();
            io::stdin().read_line(&mut input9).expect("Failed to read input");
            let length:f32 = input9.trim().parse().expect("Failed to read input");

            area_of_cube(length);


        }
        5 => {
        println!("\nenter radius");
        let mut input10 = String::new();
        io::stdin().read_line(&mut input10).expect("Failed to read input");
        let radius:f32 = input10.trim().parse().expect("Failed to read input");

        println!("enter height");
        let mut input11 = String::new();
        io::stdin().read_line(&mut input11).expect("Failed to read input");
        let height:f32 = input11.trim().parse().expect("Failed to read input");

        volume_of_cylinder(radius, height);
        }
        _ => println!("Invalid selection"),
    }


}

fn area_of_trapezium(height:f32, base1:f32, base2:f32) {
    let area:f32 = (height / 2.0) * (base1 + base2);
    println!("The area of trapezium is {}", area);
}

fn area_of_rhombus(diagonal1:f32, diagonal2:f32) {
    let area:f32 = 0.5 * diagonal1 * diagonal2;
    println!("The area of rhombus is {}", area);
}

fn area_of_parallelogram(base:f32, altitude:f32) {
    let area:f32 = base * altitude;
    println!("The area is {}", area);
}

fn area_of_cube(length:f32) {
    let area:f32 = 6.0 * length.powf(2.0);
    println!("The area is {}", area);
}

fn volume_of_cylinder(radius:f32, height:f32) {
    let pi:f32 = 22.0 / 7.0;
    let area:f32 = pi * radius.powf(2.0) * height;
    println!("The area is {}", area);
}