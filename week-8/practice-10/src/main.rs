//destructuring a tuple

fn main() {

    let b:(i32,bool,f64) = (16,true,4.9);
    print(b);
}

fn print(x:(i32,bool,f64)) {

    println!("Inside print method");
    //asigns a tuple to distinct variables
    let (age,is_female,cgpa) = x;
    println!("Age is {} , isFemale? {}, cgpa is {}",age,is_female,cgpa);
}