use std::result;

    // Approach 1
    // enum Option<T>{ //Define the generic Option type
    //     Some(T), //Represents a value
    //     None, //Represents no value
    // }

    fn divide(numerator:f64, denominator:f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    // Approach 2
    // enum Result<T, E> { //Define the generic Result type
    //     Ok(T), //Represents a value
    //     Err(E) //Represents an error
    // }

    fn divided(numerator:f64, denominator:f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Cannot divide by 0".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

fn main() {

    let result: Option<f64> = divide(10.0, 0.0);
    // match result {
    //     Some(x) => println!("Result: {}", x ),
    //     None => println!("Can't be divided by zero!"),
    // }
    match divided(28.9, 0.0){
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}