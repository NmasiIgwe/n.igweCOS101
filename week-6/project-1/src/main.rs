//rust program to qualify and accept student council voter
/*Candidate must be a class rep
  Candidate is not a 100level
  Candidate must have a CGPA of 4.0*/

use std::io;

fn main() {

    println!("Greetings Student! Welcome to Student Council Voter system. \nYou must meet certain conditions to be Eligible");

    for c in 1..151 {
        println!("You are the {}th Candidate",c);

    let mut class_rep = String::new();
    println!("Are you a Class Representative? \nEnter 1 for yes or 2 for no: ");
    io::stdin().read_line(&mut class_rep).expect("Failed to read input");
    let rep:i32 = class_rep.trim().parse().expect("Not a valid number");
    println!("assessing...");

    let mut level = String::new();
    println!("\nEnter your level: ");
    io::stdin().read_line(&mut level).expect("Failed to read input");
    let level:f32 = level.trim().parse().expect("Not a valid number");
    println!("assessing...");

    let mut cgpa = String::new();
    println!("\nEnter your CGPA: ");
    io::stdin().read_line(&mut cgpa).expect("Failed to read input");
    let cgpa:f32 = cgpa.trim().parse().expect("Not a valid number");
    println!("assessing...");

    if rep == 1 && level > 100.0 && cgpa >= 4.0
    {
        println!("You are eligible");

        let mut name = String::new();
        println!("\nEnter your name: ");
        io::stdin().read_line(&mut name).expect("Failed to read input");
        println!("assessing...");

        let mut department_name = String::new();
        println!("\nEnter the name of your department:");
        io::stdin().read_line(&mut department_name).expect("Failed to read input");

        let mut state_of_origin = String::new();
        println!("\nEnter your State of Origin:");
        io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");

        let mut email = String::new();
        println!("\nEnter your email address:");
        io::stdin().read_line(&mut email).expect("Failed to read input");

        println!("\nYou can vote");
    }
    else if rep == 2 && level <= 100.0 && cgpa < 4.0
    {
        println!("\nSorry, you are not eligible to vote");
    }
    else
    {
        println!("\nSorry, you are not eligible to vote");
    }

    println!("\n HAVE A GOOD DAY ");
}
}

         








