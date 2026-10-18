
use std::io;

fn main() {
    let mut applicants: Vec<(String, u32)> = Vec::new();

    println!("EY Nigeria Job Interview");


    loop{
        println!("\nApplicant {}", applicants.len()+1);

        println!("Enter applicant's name:");
        let name = loop {
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Failed to read input");

            if !name.trim().is_empty() {
                break name.trim().to_string();
            }

            println!("Name cannot be empty. Try again:");
        };

        let years: u32 = loop {
            println!("Enter years of programming experience:");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            match input.trim().parse::<u32>() {
                Ok(num) => break num,
                _ => println!("Invalid input. Enter a valid number."),
            }
        };

        applicants.push((name, years));
        println!("Add another applicant y/n:");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Failed to read input");

            match answer.trim().to_lowercase().as_str(){
                "y" => continue,
                "n" => break,
                _ =>println!("Must be y/n. Try again:"),
            }

            
        
    }

    let mut highest = &applicants[0];

    for applicant in &applicants {
        if applicant.1 > highest.1 {
            highest = applicant;
        }
    }

    println!("\n===== INTERVIEW RESULTS =====");

    for applicant in &applicants {
        println!("Name: {} | Experience: {} years", applicant.0, applicant.1);
    }

    println!("\nMost Experienced Applicant:");
    println!("Name: {}", highest.0);
    println!("Years of Experience: {}", highest.1);
}
