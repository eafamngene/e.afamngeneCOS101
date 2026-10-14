fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let slice1 = &numbers[1..3];
    let slice2 = &numbers[..3];
    let slice3 = &numbers[1..];
    let slice4 = &numbers[..];

    println!("Original array: {:?}", numbers);
    println!("[1..3]: {:?}", slice1);
    println!("[..3]: {:?}", slice2);
    println!("[1..]: {:?}", slice3);
    println!("[..]: {:?}", slice4);
}
