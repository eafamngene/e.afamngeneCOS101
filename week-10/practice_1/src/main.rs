// Assignment moves a Vec. The new owner is v2.
fn main() {
    let v = vec![101, 250, 330, 400];
    let v2 = v;
    println!("{:?}", v2); // v cannot be used here.
}
