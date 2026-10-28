use std::io;
use std::io::Write;

struct Laptop{
    brand: String,
    price: u32,
    quantity: u32
}

impl Laptop{
    fn purchase(&mut self, total_cost:&mut u32){

        loop{        
            print!("Quantity:");
            io::stdout().flush().expect("Failed to flush");
            let mut pur_quantity = String::new();
            io::stdin().read_line(&mut pur_quantity).expect("Invalid Input");
            let pur_quantity:u32 = pur_quantity.trim().parse().unwrap();
            if pur_quantity > self.quantity{
                println!("Enter a quantity less than {}!", self.quantity);
                continue
            }
            *total_cost += pur_quantity * self.price;
            self.quantity -= pur_quantity;
            break
        }
    }
}

fn checkout(total_cost:&u32){
    println!{"     Receipt"}
    println!("Total Cost = {}",total_cost);
}


fn main() {
    let mut total_cost = 0;
    let mut _hp = Laptop{
        brand: String::from("HP"),
        price: 650_000,
        quantity: 10,
    };
    let mut _ibm = Laptop{
        brand:String::from("IBM"),
        price: 755_000,
        quantity: 6,
    };
    let mut _toshiba = Laptop{
        brand: String::from("Toshiba"),
        price: 550_000,
        quantity: 10,
    };
    let mut _dell = Laptop{
        brand: String::from("Dell"),
        price: 850_000,
        quantity: 4,
    };

    println!("WELCOME TO ALABA ELECTRONICS\n");
    println!("WHAT WOULD YOU LIKE TO BUY");
    println!("          STOCK!!         ");
    println!("        |   {}    |   {}   |   {}   |   {}   |",_hp.brand,_ibm.brand,_toshiba.brand,_dell.brand);
    println!("PRICE   | N{} | N{} |   N{}   |  N{} | ",_hp.price,_ibm.price,_toshiba.price,_dell.price);
    println!("QUANTITY|   {}    |    {}    |      {}     |     {}    |",_hp.quantity,_ibm.quantity,_toshiba.quantity,_dell.quantity);
    loop{
        println!("\nWhat Brand:");
        print!{"1.Hp|2.IBM|3.Toshiba|4.Dell|5.menu|6.Checkout:"}  
        io::stdout().flush().expect("Failed to flush");  
         let mut pur_brand = String::new();
        io::stdin().read_line(&mut pur_brand).expect("Invalid Input");
        let pur_brand = pur_brand.trim();

        match pur_brand {
               "1"=>  _hp.purchase(&mut total_cost),
               "2"=>  _ibm.purchase(&mut total_cost),
               "3"=>  _toshiba.purchase(&mut total_cost),
               "4"=>  _dell.purchase(&mut total_cost),
               "5"=>  {println!("        |   {}    |   {}   |   {}   |   {}   |",_hp.brand,_ibm.brand,_toshiba.brand,_dell.brand);
                                     println!("PRICE   | N{} | N{} |   N{}   |  N{} | ",_hp.price,_ibm.price,_toshiba.price,_dell.price);
                                     println!("QUANTITY|   {}    |    {}    |      {}     |     {}    |",_hp.quantity,_ibm.quantity,_toshiba.quantity,_dell.quantity);}
               "6"=> break checkout(&total_cost),
              _=>continue,
          } 
    }


}
