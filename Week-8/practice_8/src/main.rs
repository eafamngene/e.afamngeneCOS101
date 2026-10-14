fn main() {
    let mut mountain_heights = ("Everest", 8848, "K2", 8611);
    println!("Before updating: {:?}", mountain_heights);

    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;
    println!("After updating: {:?}", mountain_heights);
}
