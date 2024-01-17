//rust program to solve various calculations trapezium rhombus parallelogram cube cylinder
use std::io;

fn read_f32(prompt: &str) ->f32 {
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input:f32 = input.trim().parse().expect("Not a valid number");
    return input;
} 

fn trapezium() {
    let height:f32 = read_f32("Input Height");
    let base1:f32 = read_f32("Input Base1");
    let base2:f32 = read_f32("Input Base2");

    let area = height / 2.0 * (base1+base2);
    println!("Area of Trapezium = {}",area);
} 

fn rhombus() {
    let diagonal1:f32 = read_f32("Input Diagonal1");
    let diagonal2:f32 = read_f32("Input Diagonal2");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus = {}",area);
}

fn parallelogram() {
    let base:f32 = read_f32("Input Base");
    let altitude:f32 = read_f32("Input Altitude");

    let area = base * altitude;
    println!("Area of Parallelogram = {}",area);
}

fn cube() {
    let length:f32 = read_f32("Input Length of the side");
    
    let area = 6.0 * (length.powf(2.0));
    println!("Area of Cube = {}",area);
}

fn cylinder() {
    let radius:f32 = read_f32("Input Radius");
    let height:f32 = read_f32("Input Height");

    let area = std::f32::consts::PI * (radius.powf(2.0)) * height;
    println!("Area of Cylinder = {}",area);
}

fn main() {
    println!("Select from the following shape related problems you wish to solve \nTrapezium \nRhombus \nParallelogram \nCube \nCylinder ;");
    let mut prob = String::new();
    io::stdin().read_line(&mut prob).expect("Failed to read input");
    let prob = prob.trim();


    if prob == "Trapezium"{
        trapezium();
    }else if prob == "Rhombus" {
        rhombus();
    }else if prob == "Parallelogram" {
        parallelogram();
    }else if prob == "Cube" {
        cube();
    }else if prob == "Cylinder" {
        cylinder();
    }else {
        println!("Sorry, Your problem cannot be solved");
    }
}


