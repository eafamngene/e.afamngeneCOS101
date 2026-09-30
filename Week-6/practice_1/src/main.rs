fn main() {
	let name = "Aisha Lawal";
	let university: &str = "Pan-Atlantic University";
	let address: &str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
	let department: &'static str = "Computer Science";
	let school: &'static str = "School of Science and Technology";

	println!("Name: {}", name);
	println!("University: {}", university);
	println!("Address: {}", address);
	println!("Department: {}", department);
	println!("School: {}", school);
}