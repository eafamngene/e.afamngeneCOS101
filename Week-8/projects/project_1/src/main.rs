use std::io;
//Function to check the roles and APS
fn check(sector:&[(&str,&str)])->String{
    println!("Check using:\n1. Your role\n2. Years of experience");
    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Could not read input");
        match choice.trim() {
            "1" => break,
            "2" => return check_years(sector),
            _ => println!("Please enter 1 or 2:"),
        }
    }
    println!("What role are you under:");
    //Looping through the sectors to print the roles
    for i in 0..sector.len(){
        println!("{}. {}",i+1,sector[i].0);
    }
    //Taking the input for the roles
    let role:usize=loop{
        let mut input_1=String::new();
        io::stdin().read_line(&mut input_1).expect("Could not read input");
        let num=input_1.trim().parse();
        match num{
            Ok(num) if num>=1 && num<=sector.len()=>break num,
            _=>{
                println!("Invalid input! Please choose from 1 to {}:",sector.len());
                continue;
            }
        }
    };
    //Returning the APS value for the selected role
    if sector[role-1].0==""{
        return format!("You hold the {} position.",sector[role-1].1);
    }
    return format!("You hold the {} position. Your role is {}.",sector[role-1].1,sector[role-1].0);
}
//Function to check APS using completed years of experience
fn check_years(sector:&[(&str,&str)])->String{
    println!("The table covers 1 to 13 completed years. For other years or SES, check by role.");
    let position = loop {
        println!("Enter your completed years of experience (1-13):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read input");

        //Match the input directly.
        match input.trim().parse::<u32>() {
            Ok(num) if num>=1 && num<=2 => break 0,
            Ok(num) if num>=3 && num<=4 => break 1,
            Ok(num) if num>=5 && num<=7 => break 2,
            Ok(num) if num>=8 && num<=10 => break 3,
            Ok(num) if num>=11 && num<=13 => break 4,
            Ok(num) if num>13 => break 5,
            _ => println!("Invalid input! Please enter a non negative number."),
        }
    };
    return format!("Your experience matches the {} position.", sector[position].1);
}

fn main(){
    //Declaring the vectors one for each sector
    let lawyer=vec![("Paralegal","APS 1-2"),("Junior Associate","APS 3-5"),("Associate","APS 5-8"),("Senior Associate 1-2","APS 8-10"),("Senior Associate 3-4","APS 10-13"),("Partner","SES"),];
    let office_admin=vec![("Intern","APS 1-2"),("Administrator","APS 3-5"),("Senior Administrator","APS 5-8"),("Office Manager","APS 8-10"),("Director","APS 10-13"),("CEO","SES"),];
    let academic=vec![("","APS 1-2"),("Research Assistant","APS 3-5"),("PhD Candidate","APS 5-8"),("Post-Doc Researcher","APS 8-10"),("Senior Lecturer","APS 10-13"),("Dean","SES"),];
    let teacher=vec![("Placement","APS 1-2"),("Classroom Teacher","APS 3-5"),("Senior Teacher","APS 5-8"),("Leading Teacher","APS 8-10"),("Deputy Principal","APS 10-13"),("Principal","SES"),];
    loop{
        //Printing the starting text
        println!("\nWelcome to The APS Level Checker");
        println!("What sector are you under:");
        println!("1. Lawyer\n2. OfficeAdmin\n3. Academic\n4. Teacher");
        println!("Choose from (1-4):");
        let mut input_1=String::new();
        io::stdin().read_line(&mut input_1).expect("Could not read input");
        let occupation=input_1.trim();
        //Matching each input with the correct function call
        let checked=match occupation{
            "1"=>check(&lawyer),
            "2"=>check(&office_admin),
            "3"=>check(&academic),
            "4"=>check(&teacher),
            _=>{
                println!("Invalid sector! Please choose from 1 to 4.");
                continue;
            }
        };
        println!("{}",checked);
        //Checking the restart answer
        let restart=loop{
            println!("Do you want to restart? y/n:");
            let mut answer=String::new();
            io::stdin().read_line(&mut answer).expect("Could not read input");
            let answer=answer.trim().to_lowercase();
            if answer=="y"{
                break true;
            }else if answer=="n"{
                break false;
            }else{
                println!("Invalid answer! Please enter y or n.");
            }
        };
        if restart==false{
            break;
        }
    }
}
