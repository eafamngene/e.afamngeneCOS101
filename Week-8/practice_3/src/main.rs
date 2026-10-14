use std::io;

fn main() {
    let v = vec!['R', 'U', 'S', 'T'];
    println!("Characters: {:?}", v);
    println!("Enter an index:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<usize>() {
        Ok(index) => {
            let ch = v.get(index);
            println!("Character at index {}: {:?}", index, ch);
        }
        Err(_) => println!("Please enter a non-negative integer."),
    }
}
