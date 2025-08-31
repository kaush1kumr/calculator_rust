use std::io;

fn sum(a:i32, b:i32) -> i32 {
    a+b
}
fn diff(a:i32, b:i32) -> i32 {
    a-b
}
fn product(a:i32, b:i32) -> i32 {
    a*b
}
fn division(a:i32, b:i32) -> f32 {
    a as f32 / b as f32
}

fn main() {
    loop {
        println!("This is a simple calculator implemented with rust lang.");
        println!("Please Choose One of the options:\n0) Exit \n1) Sum \n2) Difference \n3) Product \n4) Division");
        
        let mut option: String = String::new();
        io::stdin().read_line(&mut option).expect("Error taking input");
        let option:i32 = option.trim().parse().expect("ERROR IN CONVERSION");
        
        if option == 0 {
            println!("Exiting...");
            break;
        }

        let mut x: String = String::new();
        println!("Please provide first number");
        io::stdin().read_line(&mut x).expect("Error taking input");
        let x:i32 = x.trim().parse().expect("ERROR IN CONVERSION");

        let mut y: String = String::new();
        println!("Please provide second number");
        io::stdin().read_line(&mut y).expect("Error taking input");
        let y:i32 = y.trim().parse().expect("ERROR IN CONVERSION");

        match option {
            1 => println!("Result: {}", sum(x,y)),
            2 => println!("Result: {}", diff(x,y)),
            3 => println!("Result: {}", product(x,y)),
            4 => {
                if y == 0 {
                    println!("Error: Division by zero");
                } else {
                    println!("Result: {}", division(x,y));
                }
            }
            _ => println!("Invalid option"),
        }
    }
}
