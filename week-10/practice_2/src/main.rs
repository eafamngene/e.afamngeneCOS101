// Borrow to let main keep using v2 after display.
fn main() {
    let v = vec![10, 20, 30];
    let v2 = v;
    display(&v2);
    println!("In main {:?}", v2);
}
fn display(v: &[i32]) {
    println!("inside display {:?}", v);
}
