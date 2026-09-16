use std::io;
fn main() {

	println!("Principal = ",);
	let mut p = String :: new();

	io::stdin().read_line(&mut p).expect("Wrong Input");
	let p:u32= p.trim().parse().expect("Input not acceptable");
	println!("Principal = {}", p) ;

}