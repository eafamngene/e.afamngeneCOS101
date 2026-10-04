use chrono::Local;
use std::io;

fn main() {
    let customer_name = loop {
        println!("Enter the customer's name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read customer name");
        let name = name.trim().to_string();
        if !name.is_empty() {
            break name;
        }
        println!("Customer name cannot be empty.");
    };

    let mut order: Vec<(&str, u32, u32)> = Vec::new();
    loop {
        println!("\nRestaurant Menu");
        println!("P - Poundo Yam / Edinkaiko Soup   N3,200");
        println!("F - Fried Rice & Chicken         N3,000");
        println!("A - Amala & Ewedu Soup           N2,500");
        println!("E - Eba & Egusi Soup             N2,000");
        println!("W - White Rice & Stew            N2,500");

        println!("Choose P, F, A, E, W, or C to checkout:");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read food choice");
        let choice = choice.trim().to_ascii_uppercase();
        if choice == "C" {
            break;
        }

        let (food, price) = match choice.as_str() {
            "P" => ("Poundo Yam / Edinkaiko Soup", 3200),
            "F" => ("Fried Rice & Chicken", 3000),
            "A" => ("Amala & Ewedu Soup", 2500),
            "E" => ("Eba & Egusi Soup", 2000),
            "W" => ("White Rice & Stew", 2500),
            _ => {
                println!("Invalid food type.");
                continue;
            }
        };

        let quantity = loop {
            println!("Enter quantity for {food}:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read quantity");
            match input.trim().parse::<u32>() {
                Ok(quantity) if quantity > 0 => break quantity,
                _ => println!("Enter a quantity greater than zero."),
            }
        };
        order.push((food, price, quantity));
        println!("Added {quantity} x {food}.");
    }

    if order.is_empty() {
        println!("No items in your order. Goodbye!");
        return;
    }

    let subtotal: u32 = order
        .iter()
        .map(|(_, price, quantity)| price * quantity)
        .sum();
    let discount = if subtotal > 10_000 { subtotal * 5 / 100 } else { 0 };
    let total = subtotal - discount;

    println!("\nTotal to pay: N{total}");
    println!("After payment, press Enter to print the receipt.");
    let mut payment_confirmation = String::new();
    io::stdin()
        .read_line(&mut payment_confirmation)
        .expect("Failed to confirm payment");

    println!("\n========== RECEIPT ==========");
    println!("Customer: {customer_name}");
    println!("Payment time: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("-----------------------------");
    for (food, price, quantity) in &order {
        println!("{food} x {quantity}: N{}", price * quantity);
    }
    println!("-----------------------------");
    println!("Subtotal: N{subtotal}");
    println!("Discount: N{discount}");
    println!("Total paid: N{total}");
    println!("=============================");

    println!("How was your food? Anything else you'd like to share?");
    let mut feedback = String::new();
    io::stdin()
        .read_line(&mut feedback)
        .expect("Failed to read feedback");
    println!("Thank you for your feedback!");
}