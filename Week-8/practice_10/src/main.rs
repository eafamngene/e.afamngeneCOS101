fn print(x: (i32, bool, f64)) {
    let (age, is_male, cgpa) = x;
    println!("Age is {}, is male is {}, CGPA is {}", age, is_male, cgpa);
}

fn main() {
    let b = (18, true, 4.5);
    print(b);
}
