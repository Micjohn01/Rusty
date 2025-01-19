

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
