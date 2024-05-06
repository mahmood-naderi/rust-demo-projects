use std::io;
use std::cmp::Ordering;

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
}

fn bmi_calculator(height:f32, weight:f32) -> f32 {
    let height = cm_to_m(height);
    let bmi = weight / (height * height);
    bmi
}

fn cm_to_m(height:f32) -> f32 {
    return height / 100.0;
}