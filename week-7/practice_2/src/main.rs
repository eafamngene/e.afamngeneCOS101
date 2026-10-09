use std::io;

fn checker() -> io::Result<()> {
    println!("Enter one character:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut characters = input.trim_end_matches(&['\r', '\n'][..]).chars();
    let Some(character) = characters.next() else {
        println!("No character was entered.");
        return Ok(());
    };

    if characters.next().is_some() {
        println!("Please enter exactly one character.");
    } else if ('0'..='9').contains(&character) {
        println!("{character} is a digit.");
    } else {
        println!("{character} is not a digit.");
    }

    Ok(())
}

fn main() -> io::Result<()> {
    checker()
}
