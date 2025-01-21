fn main() {
    let _v:Vec<i32> = Vec::new();

    // let mut _v:Vec<i32> = vec![1,2,3];

    // _v.push(4);
    // _v.push(5);
    // _v.push(6);
    // _v.push(7);
    // _v.push(8);
    // _v.push(9);

    // println!("{:?}", _v);

    let mut _v:Vec<i32> = vec![1,2,3,4,5];

    let fourth: &i32 = &_v[3]; //Direct Indexing
    println!("The fourth element is {fourth}");

    let third = _v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no fifth element")
    };
    
}