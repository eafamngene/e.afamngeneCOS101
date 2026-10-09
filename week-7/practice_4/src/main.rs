use std::error::Error;
use std::io::{self, Write};

fn add(a: i32, b: i32) {
    println!("Sum: {}", i64::from(a) + i64::from(b));
}

fn read_number(prompt: &str) -> Result<i32, Box<dyn Error>> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let a = read_number("Enter the first number: ")?;
    let b = read_number("Enter the second number: ")?;
    add(a, b);
    Ok(())
}
