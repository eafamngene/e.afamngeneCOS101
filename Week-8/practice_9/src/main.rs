fn print(x: (i32, bool, f64)) {
    println!("Tuple: {:?}", x);
    println!("Age: {}", x.0);
    println!("Is male: {}", x.1);
    println!("CGPA: {}", x.2);
}

fn main() {
    let b = (18, true, 4.5);
    print(b);
}
