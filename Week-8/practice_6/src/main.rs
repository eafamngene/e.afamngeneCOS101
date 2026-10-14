fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x = vec![8, 7, 6, 5, 4, 3, 2, 1];

    for index in 0..v.len() {
        let sum = v[index] + x[index];
        println!("{} + {} = {}", v[index], x[index], sum);
    }
}
