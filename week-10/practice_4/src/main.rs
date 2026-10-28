// Prediction for the original: E0382, so NO lines execute.
// Fixed version borrows rather than consuming the vector.
fn main() {
    let v = vec![15, 25, 35, 45, 55];
    print_vector(&v);
    println!("v[0] = {}", v[0]);
}
fn print_vector(x: &[i32]) {
    println!("Inside print_vector function {:?}", x);
}
