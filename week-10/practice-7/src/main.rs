struct Employee {
    name:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = Employee {
        company:String::from("Igwe & Co."),
        name:String::from("Igwe Nmasi"),
        age:16
    };
    println!("Name = {} \n",emp1.name);
    println!("Company = {} \n",emp1.company);
    println!("Age = {}",emp1.age);
}