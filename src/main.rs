use functions::{add_numbers, calculate_bmi, my_id};

mod primitive_datatypes;
mod compound_datatypes;
mod functions;


fn main() {
    // println!("Hello, world!");

//     primitive_datatypes::primitive();
//     primitive_datatypes::floating();

    compound_datatypes::arrays();
    functions::tell_height(184);
    my_id("Michael", 20, 184.0);
    // print!("result: {}", functions::add_numbers(6, 9));

    let y: i32 = add_numbers(6, 9);
    println!("The sum is: {}", y);

    // let x: f64 = calculate_bmi(70.0, 1.93);
    // println!("Your BMI is: {}", x)
    let weight : f64 = 70.0;
    let height : f64 =  2.93;
    let bmi :f64 = calculate_bmi(weight, height);
    println!("Your BMI is {:.3}", bmi);
    
    let s1 = String::from("RUST");
    // let s2 = s1;
    // println!("{}", s2);
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);
}

pub fn calculate_length(s: &String) -> usize {
    s.len()
}