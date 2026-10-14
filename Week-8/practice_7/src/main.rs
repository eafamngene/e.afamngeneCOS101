fn main() {
    let t: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Typed tuple: {:?}", t);
    println!("First: {}, second: {}, third: {}", t.0, t.1, t.2);

    let t = ("Rust", "fun", 100);
    println!("Inferred tuple: {:?}", t);
    println!("First: {}, second: {}, third: {}", t.0, t.1, t.2);
}
