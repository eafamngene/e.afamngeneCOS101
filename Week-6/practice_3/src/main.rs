fn main() {
	let name1 = "Ayomide Adesokan";
	let name2 = name1.replace("Ayomide", "Adebare");
	let faculty = "Faculty of Science and Technology";
	let school = faculty.replace("Faculty", "School");

	println!("Original name: {}", name1);
	println!("New name: {}", name2);
	println!("Faculty: {}", faculty);
	println!("School: {}", school);
}