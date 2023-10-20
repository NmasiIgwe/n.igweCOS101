fn main() {
    let p:f64 = 210_000.0;
    let r:f64 = 5.0;
    let n:f64 = 3.0;

    //depreciation
    let z = 1.0 - (r/100.0);
    let w = f64::powf(z, n);
    let a = p * w;
    println!("Amount is {}", a);
    let de = a - p;
    println!("Depreciation is {}", de);

}
