// Compound Data Types
// arrays, tuples, slices, and strings (slice string)

pub fn arrays() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);
    println!("Third element: {}", numbers[2]);
    println!("Fourth element: {}", numbers[3]);
    println!("Fifth element: {}", numbers[4]);
    println!("Numbers Array: {:?} ", numbers);

    let months: [&str; 12]= ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    print!("All months: {:?}", months);

    // Tuples
    let human:(&str, bool, u8)  = ("John", true, 30);
    println!("Human: {:?} ", human);

    let my_mix_tuple = ("Michael", 23, true, [2,3,4,5]);
    println!("My mix tuple: {:?} ", my_mix_tuple);

    // Slices
    let my_slice: &[i32] = &[5,6,7,8];
    println!("Slice: {:?} ", my_slice);

    let animal_slice: &[&str] = &["cat", "dog", "fish"];
    println!("Slice: {:?} ", animal_slice);

    let book_slices: &[&String] = &[&"Harry Potter".to_string(), &"The Avengers".to_string(), &"Telekinesis".to_string()];
    println!("Slice: {:?} ", book_slices);

    // Strings Vs String Slices (&str)
    // Strings are growable, mutable, owned string types
    // String slices are immutable, borrowed string types

    let secret_agent : String = String::from("DarkMode, ");
    print!("What is our secret: {}", secret_agent);
}