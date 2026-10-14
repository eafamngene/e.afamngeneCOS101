fn main() {
    let name = vec!["Grace", "Effiong", "Emma", "David"];
    let age = vec![18, 19, 17, 20];

    for i in 0..age.len() {
        println!("{} is {} years old", name[i], age[i]);
    }
}
