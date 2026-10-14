fn main() {
    let mut colors = vec!["red", "green", "blue", "yellow"];
    println!("Original colors: {:?}", colors);

    let sliced_colors = &mut colors[1..3];
    println!("Slice before updating: {:?}", sliced_colors);
    sliced_colors[1] = "purple";
    println!("Slice after updating: {:?}", sliced_colors);

    println!("Colors after updating the slice: {:?}", colors);
}
