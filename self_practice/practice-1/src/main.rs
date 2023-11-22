//rust program to determine the eligibility of a student to join a student council 
/*conditions to be in student council: must be in atleast Year 1
                                       must be in a department and a student
                                       must have a cgpa of 3.00 above
                                    */

use std::io;

fn main() {

    println!("Greetings Student! Welcome to the student council application platform \nYou must meet certain conditions to to be eligible to join Student Council");
    let mut department_name = String::new();
    println!("Enter the name of your department:");
    io::stdin().read_line(&mut department_name).expect("Failed to read input");

    let mut year = String::new();
    println!("Enter your year:");
    io::stdin().read_line(&mut year).expect("Failed to read input");
    let year:f32 = year.trim().parse().expect("Not a valid number");
    println!("Information being assessed...");

    let mut cgpa = String::new();
    println!("Enter the value of your cgpa:");
    io::stdin().read_line(&mut cgpa).expect("Failed to read input");
    let cgpa:f32 = cgpa.trim().parse().expect("Not a valid number");
    println!("Information being assessed...");

    if year >= 1.0 && year <= 5.0 && cgpa >= 3.00
    { 
        println!("You are welcome to PAU Student Council!!! \nYou'll be notified on the meeting schedule");
    }
    else if year < 1.0 && cgpa < 3.0
    {
        println!("Sorry you did not meet up to the conditions of the student council \n Try Again later");
    }
    else {
        println!("Sorry you did not meet up to the conditions of the student council \n Try Again later");
    }
    println!("Have a Good Day \nThe PAU Student Council");
}

