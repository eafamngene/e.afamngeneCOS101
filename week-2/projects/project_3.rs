// just copied my code from project1 to here and made some twiks
fn main(){
	let principal:f64 = 210_000.0;
	let time_n:f64 = 3.0;
	let rate:f64 = 5.0;

	// Calculating the amount 
	let amount:f64 = principal * (1.0 - rate/100.0).powf(time_n);


	println!("The value of the Tv after 3 years is {}",amount.ceil() );
}