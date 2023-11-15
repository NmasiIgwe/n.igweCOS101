//a program that takes as input the experience and age of an employee to determine the annual incentive
use std::io;

fn main() {
    let p1 = 1_560_000;
    let p2 = 1_480_000;
    let _p3 = 1_300_000;
    let p4 = 100_000;

    let mut q1 = String::new();
    let mut q2 = String::new();

    println!("The employee experienced? \n(1 is yes, 2 is no)");
    io::stdin().read_line(&mut q1).expect("Not a valid string");
    let employee_experience:i32 = q1.trim().parse().expect("Not a valid number");

    println!("How old is the employee");
    io::stdin().read_line(&mut q2).expect("Not a valid string");
    let age:i32 = q2.trim().parse().expect("Not a valid number");

    if employee_experience == 2
    {
        println!("Employee incentive is N{}",p4);

    }else if age >= 40 
    {
        println!("Employee incentive is N{}",p1);
    }else if age >=30 && age < 40
    {
        println!("Employee incentive is N{}",p2);
    }else if age <= 29
    {
        print!("Employee incentive id N{}",_p3);
    }

}