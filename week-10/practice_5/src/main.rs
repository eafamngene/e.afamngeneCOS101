fn main() {
    let x = vec![10, 20, 30];
    borrow_vector(&x); // Shared reference; x remains the owner.
    println!("Printing the value from main() x[0]={}", x[0]);
    println!("--------------------------");
}
// Matches the slide. &[i32] is a more flexible alternative for read-only access.
fn borrow_vector(z: &Vec<i32>) {
    println!("Inside print_vector function {:?}", z);
    println!("--------------------------");
}
