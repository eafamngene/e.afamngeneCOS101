use std::io;

fn main() {
    let mut running = true;

    while running {
        let mut exp_input = String::new();
        let mut age_input = String::new();
        let mut choice = String::new();

        println!("\nIs the employee experienced? (yes/no): ");
        io::stdin().read_line(&mut exp_input).expect("Failed to read line");
        let is_experienced = exp_input.trim().to_lowercase();

        if is_experienced == "yes" || is_experienced == "y" {
            println!("Enter employee age: ");
            io::stdin().read_line(&mut age_input).expect("Failed to read line");
            let age: u32 = age_input.trim().parse().expect("Please type a valid number");

            if age >= 40 {
                println!("Annual Incentive: N1,560,000");
            } else if age >= 30 && age <= 39 {
                println!("Annual Incentive: N1,480,000");
            } else if age < 28 {
                println!("Annual Incentive: N1,300,000");
            } else {
                println!("No incentive criteria specified for ages 28-29.");
            }
        } else {
            println!("Annual Incentive: N100,000");
        }

        println!("\nDo you want to check another employee? (yes/no): ");
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let answer = choice.trim().to_lowercase();
        if answer != "yes" && answer != "y" {
            println!("Goodbye!");
            running = false;
        }
    }
}