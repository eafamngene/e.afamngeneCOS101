fn main() {
    let arr1: [i32; 4] = [10, 20, 30, 40];
    let arr2 = [10.4, 20.7, 30.1, 40.9];
    let arr3: [i32; 8] = [0; 8];

    println!("arr1: {:?} (length: {})", arr1, arr1.len());
    println!("arr2: {:?} (length: {})", arr2, arr2.len());
    println!("arr3: {:?} (length: {})", arr3, arr3.len());
}
