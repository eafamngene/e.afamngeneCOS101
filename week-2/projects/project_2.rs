// sir i had to go and learn the syntax for for loops and everything else is almost similar to python so i'm good
fn main() {
    let quantity = [2, 1, 3, 3, 1];
    let amount = [450000, 1500000, 750000, 2850000, 250000];
    // i learnt that you use mut for when you want to be able to change the variable agains sha so i needed to use it in this context
    let mut sum = 0;

    // just a basic for loop in rust syntax nothing hard
    // this just loop through each quantity and amount witht the same index and multiplies them
    for i in 0..quantity.len() {
        sum += quantity[i] * amount[i];
    }

    // find the average by divding the sum my total length of quantity 
    let average = sum / quantity.len() ;

    // then just printing the code
    println!("Sum: {}", sum);
    println!("Average: {}", average);
}