fn main() {
	let a = 20;
	let b = 30;
	println!("(a > 10) && (b > 10): {}", (a > 10) && (b > 10));

	let c = 0;
	let d = 30;
	println!("(c > 10) || (d > 10): {}", (c > 10) || (d > 10));

	let is_elder = false;
	println!("!is_elder: {}", !is_elder);
}