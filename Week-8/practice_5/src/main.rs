use std::io;

fn main() {
    let mut city: Vec<String> = Vec::new();
    println!("How many preferred cities would you like to enter?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let count: usize = match input.trim().parse() {
        Ok(count) => count,
        Err(_) => {
            println!("Please enter a non-negative whole number.");
            return;
        }
    };

    for i in 0..count {
        println!("Enter preferred city {}:", i + 1);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let new_city = input.trim().to_string();
        city.push(new_city);
    }

    println!("Your preferred cities: {:?}", city);
    println!("Number of cities: {}", city.len());
}
