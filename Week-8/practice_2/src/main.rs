use std::io;

fn main() {
    let v = vec!['R', 'U', 'S', 'T'];
    println!("Characters: {:?}", v);
    println!("Enter an index (0 to {}):", v.len() - 1);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<i64>() {
        Ok(index) if index >= 0 && (index as usize) < v.len() => {
            let ch: char = v[index as usize];
            println!("Character at index {}: {}", index, ch);
        }
        Ok(_) => println!("Index is out of bounds."),
        Err(_) => println!("Please enter a valid integer."),
    }
}
