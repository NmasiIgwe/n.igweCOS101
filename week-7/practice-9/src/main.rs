//array with for loop using iter() function

fn main() {
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is : {}",arr.len());

    for value in arr.iter() {
        println!("value is : {}",value);
    }
}