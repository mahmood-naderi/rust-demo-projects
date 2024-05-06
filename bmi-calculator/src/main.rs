use std::io;
use colored::*;

fn main() {
    println!("Welcome to BMI calculator!");
    println!("Enter your weight in kg:");
    let mut weight: String = String::new();
    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read the weight!");    
    println!("Enter your height in cm:"); 
    let mut height: String = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read the height!");

    
    let weight: f32 = match weight.trim().parse() {
        Ok(weight) => weight,
        Err(_) => 0.0,
    };
    let height: f32 = match height.trim().parse() {
        Ok(height) => height,
        Err(_) => 0.0,
    };

    let bmi = bmi_calculator(height, weight);
    println!("Your BMI is {}", bmi);
    print_message(bmi);
}

fn bmi_calculator(height:f32, weight:f32) -> f32 {
    let height = cm_to_m(height);
    let bmi = weight / (height * height);
    bmi
}

fn cm_to_m(height:f32) -> f32 {
    return height / 100.0;
}

fn print_message(bmi:f32) {
    if bmi >= 30.0 { println!("{}", "You are overweight!".red()); }
    else if bmi <= 18.4 { println!("{}", "You are underweight!".yellow()); }
    else { println!("{}", "You are in normal weight!".blue()); }
}