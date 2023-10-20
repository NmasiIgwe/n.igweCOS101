fn main() {
    let tosh = 450_000.0;
    let mac = 1_500_000.0;
    let hp = 750_000.0;
    let dell = 2_850_000.0;
    let acer = 250_000.0;
    
    // average
    let s = (2.0 * tosh) + (1.0 * mac) + (3.0 * hp) + (3.0 * dell) + (1.0 * acer);
    println!("Sum is {}", s);
    let ave = s / 10.0;
    println!("Average is {}", ave);

}
    