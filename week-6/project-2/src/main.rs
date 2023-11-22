//rust program to display name and incentive for researcher and their papers they published in NRG

use std::io;

fn main() {
    for _n in 1..501 {

    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let mut papers = String::new();
    println!("Enter the number of papers published: ");
    io::stdin().read_line(&mut papers).expect("Failed to read input");
    let papers:f32 = papers.trim().parse().expect("Not a valid number");

        if papers >= 3.0 && papers <= 5.0
        {
            println!(" {}, your incentive is N500,000",name);
        }
        else if papers >= 5.0 && papers <= 10.0
        {
            println!(" {}, your incentive is N800,000",name);
        }
        else if papers >= 10.0
        {
            println!(" {}, your incentive is N1,000,000",name);
        }
        else 
        {
            println!(" {}, your incentive is N100,000",name);
        }
    }    
}   