use std::io;

fn main(){

	loop{

		let mut number = String::new();
		
		println!("Enter a number:");
		io::stdin().read_line(&mut number).unwrap();

		let number_int : i32 = number.trim().parse().unwrap();
		println!("The number {}", number_int);
		if number_int == 0 {
			break
		}
	}



	
     
}