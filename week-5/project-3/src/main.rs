//rust program to display menu for the food items available to take order from the customer

use std::io;

fn main() {
    let mut total:f32 = 0.0;

    println!("\t\tMenu\t\t\tPrice");
    println!("P = Poundo Yam/Edinkaiko Soup\t  -N3,200");
    println!("F = Fried Rice & Chicken   -N3,000");
    println!("A = Amala & Ewedu Soap\t   -N2,500");
    println!("E = Eba & Egusi Soup\t     -N2,000");
    println!("W = White Rice & Stew\t    -N2,500");

    println!("Enter letter of food you would like like to order (q to quit/Submit order):");
    loop {
        let mut food = String::new();
        io::stdin().read_line(&mut food).expect("Failed to read input");
        let food = food.trim();

        if food == "P"{
            total += 3200.0;
        }else if food == "F"{
            total += 3000.0;
        }else if food == "A"{
            total += 2500.0;
        }else if food == "E"{
            total += 2000.0;
        }else if food == "W"{
            total += 2500.0;
        }else if food == "q"{
            break;
        }else { 
            println!("Sorry, we dont have  that here");
            continue;
        }
    }
}