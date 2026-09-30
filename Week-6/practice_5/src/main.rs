fn main() {
	let fullname = " Pan-Atlantic University ";

	println!("Before trim: {:?}", fullname);
	println!("Length before trim: {}", fullname.len());
	println!("After trim: {:?}", fullname.trim());
	println!("Length after trim: {}", fullname.trim().len());
}