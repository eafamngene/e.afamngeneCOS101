fn main() {
    let v = vec![101, 250, 330, 400];
    let v2 = v;
    println!("{:?}", v); // Expected E0382: v was moved to v2.
}
