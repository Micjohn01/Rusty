// Loop
// While Loop
// For Loop

fn main () {
    // let mut counter = 0;

    // let result: i32 = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // While Loops
    let mut count = 0;
    while count < 10 {
        println!("count: {count}");
        count += 1;
    }

    let mut number: i32 = 3;
    while number != 0 {
        println!("{number}");
        number -=1;
        // break;
    
    }

    // For Loop
    let a: [i32; 6] = [1,2,3,4,5,6];
    let b: [&str; 4] = ["a", "b", "c", "d"];

    for element in a {
        println!("{element}");
    }

    for letters in b{
        println!("{letters}");
    }
}