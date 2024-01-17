//read from a file using open()
//first i created a file from notepad called welcome_message.txt

use std::io::Read;

fn main(){
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}