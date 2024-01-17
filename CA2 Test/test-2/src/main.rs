//rust program to store some certain information for some companies and calculate their leverages, liabilities and so on
use std::io;
use std::io::Read;

fn validate_password(password:&str) -> bool {
    //this function checks if the password meets the requirements and returns a boolean true or false
    // Minimum characters is 3 && max is 8
    println!("Checking password {}", password);
    if password.len() < 3 || password.len() > 8 {
        return false;
    }
    // Only lowercase or numbers
    for ch in password.chars() {
        if !ch.is_lowercase() && !ch.is_ascii_digit() {
            return false;
        }
    } 
    return true;
}

fn login() -> String {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your company username: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let username:String = input1.trim().to_string();

    if username.len() != 4 {
        panic!("Username must be 4 letters");
    }

    println!("\nEnter your password: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let password = input2.trim();
    let is_valid : bool = validate_password(password);
    if is_valid {
        println!("Valid Password \nWELCOME!!!");
    }else {
        println!("Invalid Password");
    }
    return username;
}

//create the struct that will help read the data
struct CompanyData {
    name:String,
    assests:i64,
    liabilities:i64,
    year:i64,
} 

fn leverage(company:&CompanyData)-> f64 {
    let diff = (company.assests - company.liabilities) as f64;
    let leverage = diff /  company.assests as f64 ;
    return leverage * 100.0; 
} 

fn read_companydata() -> Vec<CompanyData> {
    let mut file = std::fs::File::open("CompanyData.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    //println!("{}", data);
    let info: Vec<&str> = data.split("\n").collect();
   // println!("{:?}", info);

    let mut companies_data : Vec<CompanyData> = Vec::new();
    for line in info {
        let line: Vec<&str> = line.split(",").collect();
        //in the vector line it contains: line = [name, assests, liabilites, year]
        //println!("{:?}", line);


        let name:&str = line[0].trim();
        let assests:i64 = line[1].trim().parse().expect("Invalid Assests");
        let liabilities:i64 = line[2].trim().parse().expect("Invalid Liabilities");
        let year:i64 = line[3].trim().parse().expect("Invalid Year");
        let company_data = CompanyData {name:  name.to_string(), assests, liabilities, year};
        companies_data.push(company_data);
    }
    return companies_data;
}
fn main() {
    let username = login();
    let companies_data = read_companydata();

    for company in companies_data {
        let company_leverage = leverage(&company);

        if company.name.to_lowercase().starts_with(&username) {
            if company.assests > 20000000{
                 println!("\n {} has leverage of {}",company.name,company_leverage);  
            }
            if company.liabilities < 10000000 {
                 println!("\n {} has leverage of {}",company.name,company_leverage*0.05 );
            }
             println!("\n {} has assests of N{}",company.name, company.assests);
            println!("\n {} has liabilities of N{}",company.name, company.liabilities);
            println!("\n {} was founded in year {}",company.name, company.year);
        } 
    }
}