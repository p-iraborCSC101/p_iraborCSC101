use std::io;

fn main() {

    let mut rootA:f32 = 0.0;
    let mut rootB:f32 = 0.0;

    let mut realp:f32 = 0.0;
    let mut imagp:f32 = 0.0;
    let mut D:f32  = 0.0;

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input2.trim().parse().expect("Not a valid number");

    
     if a == 0.0 || b == 0.0 || c == 0.0 
    {
        println!("Error: Unable to determine roots");
    }
    else {
        D = b * b - (4.0 * a * c);
        if D < 0.0 {
            println!("Roots do not exist, they are imaginary");
            realp = -b / (2.0 * a);
            D = D.abs();
            imagp = D.sqrt() / (2.0 * a);
            println!("Root1 = {}  +i {}", realp, imagp);
            println!("Root2 = {}  -i {}", realp, imagp);
        }
        else if D > 0.0 {
           rootA = (-b + D.sqrt()) / (2.0 * a);
           rootB = (-b - D.sqrt()) / (2.0 * a);
            println!("Root1 = {}  ", rootA);
            println!("Root2 = {}  ", rootB);
        }
        else if D == 0.0 {
            rootA = -b / (2.0 * a);
            rootB = rootA;
            println!("Root1 = {}", rootA);
            println!("Root2 = {}", rootB);
        }
    }


}

        


