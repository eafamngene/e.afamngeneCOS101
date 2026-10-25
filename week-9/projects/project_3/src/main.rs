use std::io::Write;


fn main() {
    let commissioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zone = vec!["South West","North East","South South","South West","South East"];

    let mut file = std::fs::File::create("convicted_ministers.csv").unwrap();
    
    file.write_all("Commissioner,Ministry,Zone\n".as_bytes()).expect("Failed to write");

    for i in 0..commissioner.len(){
        let mut b = "";
        let mut c = "";

        let a = commissioner[i];
        if i < ministry.len(){
            b = ministry[i];
        }
        if i < zone.len(){
            c = zone[i];
        }
        let data = format!("{},{},{}\n",a,b,c);
        file.write_all(data.as_bytes()).expect("Failed to Write");
    }

    println!("File created Succefully");
    
}
