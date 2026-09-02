fn main() {
	let principal:f64 = 520_000_000.0;
	let time_n:f64 = 5.0;
	let rate:f64 = 10.0;

	// Calculating the amount 
	let amount:f64 = principal * (1.0 + rate/100.0).powf(time_n);
	// Calculating the compound interest
	let compound_int:f64 = amount - principal;
	// I'm just trying to over do buh let me round the number up lol.
	let compound_int_rounded = compound_int.ceil();

	println!("The compount interest is {}",compound_int );
	print!("Rounded up to {}", compound_int_rounded);
}