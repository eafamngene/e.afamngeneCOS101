// importing the std library
use std::io;

fn main() {
    println!("WELCOME TO THE QUADRATIC CALCULATOR");

    loop{

        //Declearing the variables
        let mut _a = String::new();
        let mut _b = String::new();
        let mut _c = String::new();
        let mut root1;
        let mut root2;

        // Printing the starting texts
        println!("\nEnter the values of :");

        //Taking all the necessary inputs
        println!("a: ");
        io::stdin().read_line(&mut _a).expect("Failed to read input");
        let _a:f32 = _a.trim().parse().expect("Invalid number ");

        println!("\nb: ");
        io::stdin().read_line(&mut _b).expect("Failed to read input");
        let _b:f32 = _b.trim().parse().expect("Invalid number ");

        println!("\nc: ");
        io::stdin().read_line(&mut _c).expect("Failed to read input");
        let _c:f32 = _c.trim().parse().expect("Invalid number ");

        let _d = _b*_b - 4.0*_a*_c;

        // filtering 
        if _d > 0.0{
            // Calculate and print two distinct real roots
            root1 = ((-1.0*_b) + _d.sqrt())/(2.0*_a);
            root2 = ((-1.0*_b) - _d.sqrt())/(2.0*_a);

            println!("\nROOT = {},{}",root1,root2 );
        }else if _d == 0.0 {
            //Calculate and print the single real root 
            root1 = (-1.0*_b)/(2.0*_a);
            println!("\nROOT = {}",  root1);
        }else{
            println!("\nNo real roots exist for this equation");
        }

        // Ask the user if they want to calculate another equation
        println!("\nWould you like to calculate another? (y/n): ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        // Break out of the loop if the answer isn't 'y' or 'Y'
        if choice.trim().to_lowercase() != "y" {
            println!("Goodbye!");
            break;
        }


    }


}
