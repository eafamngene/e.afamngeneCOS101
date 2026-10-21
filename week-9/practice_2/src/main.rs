use std::io::Read;
use std::io::Write;
fn main() {
    let message = "The world is going to end one day";
    let mut file = std::fs::File::create("welcome_message.txt").unwrap();
    file.write_all(message.as_bytes()).expect("Failed to Write");
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
