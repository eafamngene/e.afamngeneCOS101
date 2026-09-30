fn main() {
	let n1 = "Electrical".to_string();
	let n2 = "Electronic".to_string();
	let n3 = "Engineering".to_string();
	let n4 = n1 + " / " + &n2 + " / " + &n3;

	println!("{}", n4);
}