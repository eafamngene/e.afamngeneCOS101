fn add_one(e: &mut i32) {
    *e += 1; // Dereference to edit the original integer.
}
fn main() {
    let mut i = 3;
    add_one(&mut i);
    println!("i = {}", i);
}
