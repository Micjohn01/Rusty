fn main() {
    let x: i32 = 8;

    let x: i32 = x + 2;

    let x: i32 = 450;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in main function is: {x}");
}