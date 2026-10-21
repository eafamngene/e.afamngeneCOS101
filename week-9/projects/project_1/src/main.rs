use std::io::Write;


fn main() {
    let lager = vec!["33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
    let stout = vec!["Legend","Turbo King","Williams"];
    let non_Alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    let mut file = std::fs::File::create("Breweries_data.csv").unwrap();
    
    file.write_all("Lager,Stout,Non-Alchoholic\n".as_bytes()).expect("Failed to write");

    for i in 0..lager.len(){
        let mut b = "";
        let mut c = "";

        let a = lager[i];
        if i < stout.len(){
            b = stout[i];
        }
        if i < non_Alcoholic.len(){
            c = non_Alcoholic[i];
        }
        let data = format!("{},{},{}\n",a,b,c);
        file.write_all(data.as_bytes()).expect("Failed to Write");
    }

    println!("File created Succefully");
    
}
