//program to show how fast a car moves

use std::io;

fn main() {
       let mut distance = String::new();
       let mut  time = String::new();

       println!("Enter distance in miles: ");
       io::stdin().read_line(&mut distance).expect("Not a valid string");
       let distance:f64 = distance.trim().parse().expect("Not a valid number");

       println!("Enter time in hours");
       io::stdin().read_line(&mut time).expect("Not a valid string");
       let time:f64 = time.trim().parse().expect("Not a valid number");

       let speed:f64 = (distance*1.60934)/time; 
       
       println!("Speed of a car = {}km/h.",speed );
}