fn mutate_num_to_zero(mut param_num: i32) {
    param_num *= 0;
    println!("param_num is {param_num}");
}

fn main() {
    let num = 5;
    mutate_num_to_zero(num);
    println!("num is still {num}");
}
