// This lets the program read input from the keyboard.
use std::io;

// Project 1: Shape calculator
fn trapezium() -> f64 {
    // Empty strings to store the two bases and height entered by the user.
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the first base:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    // trim removes extra spaces and the newline, then parse converts the text to a number.
    let base1: f64 = input1.trim().parse().expect("Enter a number");

    println!("Enter the second base:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let base2: f64 = input2.trim().parse().expect("Enter a number");

    println!("Enter the height:");
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read input");
    let height: f64 = input3.trim().parse().expect("Enter a number");

    // Area of a trapezium = half the height multiplied by the sum of the bases.
    return height / 2.0 * (base1 + base2);
}

fn rhombus() -> f64 {
    // A rhombus needs its two diagonals to calculate the area.
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the first diagonal:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let diagonal1: f64 = input1.trim().parse().expect("Enter a number");

    println!("Enter the second diagonal:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let diagonal2: f64 = input2.trim().parse().expect("Enter a number");

    // Area = half of the product of the diagonals.
    return 0.5 * diagonal1 * diagonal2;
}

fn parallelogram() -> f64 {
    // Read the base and perpendicular height, which is the altitude.
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the base:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Enter a number");

    println!("Enter the altitude:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let altitude: f64 = input2.trim().parse().expect("Enter a number");

    // Return the area to main so it can be printed.
    return base * altitude;
}

fn cube() -> f64 {
    // All the sides of a cube have the same length, so only one input is needed.
    let mut input = String::new();

    println!("Enter the side length:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let side: f64 = input.trim().parse().expect("Enter a number");

    // A cube has six square faces, so its surface area is 6 times side squared.
    return 6.0 * side * side;
}

fn cylinder() -> f64 {
    let mut input1 = String::new();
    let mut input2 = String::new();
    // Use 22 divided by 7 for pi, like the example in class.
    let pi: f64 = 22.0 / 7.0;

    println!("Enter the radius:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let radius: f64 = input1.trim().parse().expect("Enter a number");

    println!("Enter the height:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let height: f64 = input2.trim().parse().expect("Enter a number");

    // Volume of a cylinder = pi times radius squared times height.
    return pi * radius * radius * height;
}

fn main() {
    // Store the six menu options in one array.
    let menu: [&str; 6] = [
        "1. Trapezium area",
        "2. Rhombus area",
        "3. Parallelogram area",
        "4. Cube surface area",
        "5. Cylinder volume",
        "0. Exit",
    ];

    // Keep showing the menu so another calculation can be done.
    loop {
        println!("SHAPE CALCULATOR");
        // Go through the array and print each option.
        for option in menu.iter() {
            println!("{}", option);
        }
        println!("Use the same unit for all dimensions.");
        println!("Enter your choice (0 to 5):");

        // Read the user's menu choice.
        let mut input = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        // Stop if the input has ended.
        if bytes_read == 0 {
            break;
        }
        let choice = input.trim();

        // Call the function for the selected shape and print its answer.
        if choice == "1" {
            println!("Area of the trapezium = {} square units", trapezium());
        } else if choice == "2" {
            println!("Area of the rhombus = {} square units", rhombus());
        } else if choice == "3" {
            println!(
                "Area of the parallelogram = {} square units",
                parallelogram()
            );
        } else if choice == "4" {
            println!("Surface area of the cube = {} square units", cube());
        } else if choice == "5" {
            println!("Volume of the cylinder = {} cubic units", cylinder());
        } else if choice == "0" {
            // break ends the loop when the user chooses to exit.
            println!("Goodbye!");
            break;
        } else {
            // An invalid choice shows this message, then the menu appears again.
            println!("Invalid choice. Please choose a number from 0 to 5.");
        }
        println!();
    }
}
