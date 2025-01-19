pub fn tell_height(height: u32) {
    println!("My height is: {} cm.", height);
} 

pub fn my_id(name: &str, age: u32, height:f32) {
    println!("Hello, my name is {}, I am {} years old, and I am {} cm.", name, age, height);
    
    let x: i32 = {
        let price: i32 = 10;
        let quantity: i32 = 120;
        price * quantity
    };
    println!("Total: {}", x);
}

// Expression and Statements
// Expression: Anything that returns a value.
// Statement: Anything that does not return a value.

// Expression
// -------------
// 47
// true and false
// add(9,3)
// if condition {value1} else {value2}
// ({code})

pub fn add_numbers(a: i32, b: i32) -> i32{
    a + b
}

// Statement
// let x = let y = 10;

pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}