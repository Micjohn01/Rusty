fn main() {
    let s: String = "whatever".to_string();
    let s: String = String::from("whatever");

    let mut s: String = String::from("mic");

    s.push_str("hael");
    s.push('!');

    println!("The value of s = {}", s);

    let s1: String = String::from("Sweet");
    let s2: String = String::from("heart");
    let s3: String = s1 + &s2;

    println!("The value of S3 = {}", s3);

    let first =  String::from("Fashion");
    let second =  String::from("Designer");

    let full_text = format!("{first} {second}");
    println!("{full_text}");
    
}