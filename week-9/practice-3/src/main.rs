//delete a file 
//first creat a file called data.txt

use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}