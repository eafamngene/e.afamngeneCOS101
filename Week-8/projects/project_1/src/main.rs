use std::io;
// function to  check the roles and APS
fn check(sector: &[(&str,&str)])->String{
    println!("What role are you under:");
    //Looping thorugh the sectors to print the roles
    for i in 0..sector.len(){
        println!("{}.{}",i+1,sector[i].0,)
    }

    //Taking the input for the roles
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Wrong Input");
    let role:usize= input_1.trim().parse().expect("Invalid Data type");

    //Returning the APS value for the role
    for i in 0..sector.len(){
        if (i+1) == role{
            return format!("You hold the {} position as an {}", sector[i].1,sector[i].0)
        }
    }
    String::from("Role not found")
}

fn main() {


    loop{
        //Printing the Starting Text
        println!("\nWelcome to The APS Level Checker");
        println!("What Sector are you under:");
        println!("1.Lawyer\n2.OfficeAdmin\n3.Academic\n4.Teacher");
        println!("Choose from (1-4):");
        
        let mut input_1 = String::new();
        io::stdin().read_line(&mut input_1).expect("Wrong Input");
        let occupation = input_1.trim().to_uppercase();


        //Declearing the Vectors for each sector
        let lawyer = vec![
        ("Paralegal","APS 1-2"),
        ("Junior Asssociate","APS 3-5"),
        ("Associate","APS 5-8"),
        ("Senoir Associate 1-2","APS 8-10"),
        ("Senoir Associate 3-4","APS 10-13"),
        ("Partner","SES"),
        ];
        let office_admin = vec![
        ("Intern","APS 1-2"),
        ("Administrator","APS 3-5"),
        ("Senior Administrator","APS 5-8"),
        ("Office Manager","APS 8-10"),
        ("Director","APS 10-13"),
        ("CEO","SES"),
        ];
          let academic = vec![
        ("","APS 1-2"),
        ("Research Assistant","APS 3-5"),
        ("PhD Candidate","APS 5-8"),
        ("Post-Doc Resercher","APS 8-10"),
        ("Senior Lecturer","APS 10-13"),
        ("Dean","SES"),
        ];
        
        let teacher = vec![
        ("Placement","APS 1-2"),
        ("Classroom Teacher","APS 3-5"),
        ("Senior Teacher","APS 5-8"),
        ("Leading Teacher","APS 8-10"),
        ("Deputy Principal","APS 10-13"),
        ("Principal","SES"),
        ];


    //Matching each input with the correct function call
        let checked = match occupation.as_str(){
            "1" => check(&lawyer),
            "2" => check(&office_admin),
            "3" => check(&academic),
            "4" => check(&teacher),
            _ => continue
        };

        println!("{}", checked);
        println!("Do you want to restart y/n:");
        let mut restart = String::new();
        io::stdin().read_line(&mut restart).expect("wrong value");
        let restart = restart.trim();
        if restart== "y"{
            continue;
        }else if restart == "n"{
            break
        }
    }
}