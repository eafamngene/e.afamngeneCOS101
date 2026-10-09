fn mutate_num_to_zero(param_num: &mut i32) {
    *param_num *= 0;
    println!("param_num is {param_num}");
}

fn main() {
    let mut num = 5;
    mutate_num_to_zero(&mut num);
    println!("num is {num}");
}
