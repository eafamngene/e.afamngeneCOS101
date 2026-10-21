use std::fs;
use std::io::Write;
fn main() {
    let file = std::fs::File::create("data.txt").expect("File creation failed");
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
