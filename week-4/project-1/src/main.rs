// a rust program that tells howfast a car is travelling 

fn main() {
    let d1 = 80.0 ;
    let d2 = 120.0 ;
    println!("Distance1 = {} \nDistance2 = {}",d1,d2 );

    let t1 = 2.0 ;
    let t2 = 4.0 ;
    println!("Time1 = {} \nTime2 = {}",t1,t2);

    let s1 = d1/t1;
    let s2 = d2/t2;
    println!("Speed1 = {} \nSpeed2 = {}",s1,s2 );
}