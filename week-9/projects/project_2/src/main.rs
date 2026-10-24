use std::io;
use std::io::Write;
use std::fs::OpenOptions;
fn main() {
    println!("STUDENT MANAGEMENT INFORMATION SYSTEM RECORDS");
    let valid_departments = ["accounting", "business administration", "economics", "finance", "mass communication", "information science and media studies", "film and multimedia studies", "strategic communication", "computer science", "mechanical engineering", "electrical/electronic engineering", "software engineering", "mechatronics engineering", "data science"];
    let valid_level = ["100","200","300","400"];
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("students.csv")
    .expect("Unable to open file");
    file.write_all(format!("Student Name,Matric.Number,Department,Level\n").as_bytes()).expect("Unable to save student");
    loop {
        
        println!("STUDENTS NAME: ");
        let mut stud_name = loop{
            let mut stud_name = String::new();
            io::stdin().read_line(&mut stud_name).expect("Invalid name");
            stud_name = stud_name.trim().to_string();
            match stud_name.is_empty(){
                false=> break stud_name,
                _=>{
                    println!("Enter the name of a student");
                    continue;
                }
            }
        };
        println!("\nMatric-Number: ");
        let mut matric_num = loop{
            let mut matric_num = String::new();
            io::stdin().read_line(&mut matric_num).expect("Invalid Input");
            matric_num = matric_num.trim().to_string();

            if matric_num.len() == 11{ 
                break matric_num.to_uppercase();
            } else {
                println!("Must be 11 characters");
                continue;
            }

        };
        println!("\nDepartment: ");
        let mut department = loop{
            let mut department = String::new();
            io::stdin().read_line(&mut department).expect("Invalid Input");
            department = department.trim().to_string();
            match valid_departments.contains(&department.to_lowercase().as_str()){
                true => break department,
                _=>{
                    println!("Enter a valid department");
                    continue;
                }
            }
        };
        println!("\nLevel: ");
        let mut lvl = loop{
            let mut lvls = String::new();
            io::stdin().read_line(&mut lvls).expect("Invalid Input");
            lvls = lvls.trim().to_string();

            match valid_level.contains(&lvls.as_str()){
                true => break lvls,
                _=>{
                    println!("Enter a valid level");
                    continue;
                }
            }
        };

        let student_record = format!("{},{},{},{}\n",stud_name, matric_num, department, lvl);
        file.write_all(student_record.as_bytes()).expect("Unable to save student");


        println!("\nAdd another student (a) or Exit (x):");
        let mut input_1 = loop{
            let mut input_1 = String::new();
            io::stdin().read_line(&mut input_1).expect("Invalid name");
            input_1 = input_1.trim().to_string().to_lowercase();
            match input_1.is_empty(){
                false=> break input_1,
                _=>{
                    println!("Enter a or x");
                    continue;
                }
            }
        };

        if input_1 == "x"{
            break;
        }

    }
}
