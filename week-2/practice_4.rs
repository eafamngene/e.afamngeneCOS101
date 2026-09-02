fn main() {
    let money: f64 = 1000.0;
    let percent: f64 = 1.0;
    let years: f64 = 2.0;

    // simple interest
    let total = money * (1.0 + (percent / 100.0)) * years;

    println!("Amount is {}", total);

    let profit = total - money;

    println!("Simple Interest is {}", profit);
}