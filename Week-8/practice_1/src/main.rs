fn main() {
    let v: Vec<i64> = Vec::new();
    println!("Empty vector: {:?}", v);
    println!("Length: {}", v.len());

    let v = vec!["Grace", "Effiong", "Emma", "David"];
    println!("Names: {:?}", v);
    println!("Length: {}", v.len());
}
