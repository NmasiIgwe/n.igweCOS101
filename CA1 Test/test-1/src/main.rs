/* rust program that asks for patient info 
and applies to specific condition and and apply specific discounts*/

use std::io;

fn main() {
    println!("\nOtunene Family Health Centre \nHow can we help you today");

    //Patient info 
    println!("\nPlease Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Patient name: {}",name);

    println!("\n Please Enter your date of birth:");
    let mut dob = String::new();
    io::stdin().read_line(&mut dob).expect("Failed to read input");
    let dob:f32 = dob.trim().parse().expect("Not a valid number");
    println!("Patient Date of Birth: {}",dob);

    println!("\n Please Enter your email address:");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read input");
    println!("Patient Email Address: {}",email);

    println!("\n Please Enter your phone number:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let num:i64 = num.trim().parse().expect("Not a valid number");
    println!("Patient Phone number: {}",num);

    println!("\n Please Enter your number of siblings:");
    let mut sib = String::new();
    io::stdin().read_line(&mut sib).expect("Failed to read input");
    let sib:i64 = sib.trim().parse().expect("Not a valid number");
    println!("Number of siblings of Patient: {}",sib);

    println!("\n Please Enter your number of children:");
    let mut child = String::new();
    io::stdin().read_line(&mut child).expect("Failed to read input");
    let child:i64 = child.trim().parse().expect("Not a valid number");
    println!("Number of children of Patient: {}",child);

    println!("\t\tHealth Diagnosis\t\t\tAmount");
    println!("A= Alzhelmer\t -N1,200,000");
    println!("Ar= Arrhythmia\t -N550,000");
    println!("C= Chronic Kidney Disease\t  -N1,500,000");
    println!("D= Diabetes\t  -N800,000");
    println!("Art= Athritis\t  -N450,000");

    println!("\n Please Enter your Medical Diagnosis");
    let mut dia = String::new();
    io::stdin().read_line(&mut dia).expect("Failed to read input");
    println!("We will have a nurse attend to you shortly");
   
    println!("\t\tVillage\t\t\tDiscount");
    println!("A2 = Akpabom\t   -20%");
    println!("N = Ngbauji\t   -5%");
    println!("A3 = Atabrikang\t   -15%");
    println!("O = Okorobilom\t  -10%");
    println!("E = Emeremen\t  -10%");

    println!("\n Please Enter your village of residence");
    let mut vill = String::new();
    io::stdin().read_line(&mut vill).expect("Failed to read input");
    println!("We will have a nurse attend to you shortly");
}

