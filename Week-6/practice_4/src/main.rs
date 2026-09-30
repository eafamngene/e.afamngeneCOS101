fn main() {
	let mut school = String::from("School of Science");
	school.push_str(" and Technology");

	println!("School: {}", school);
	println!("Length: {} bytes", school.len());
}