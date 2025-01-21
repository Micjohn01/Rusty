#![allow(warnings)]

fn main() {
    // tuple
    let rectangle:(i32, i32) = (200, 500);

    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    let mut user1: User = User{
        active: true,
        username: String::from("uniqueusername"),
        email: String::from("unique@gmail.com"),
        sign_in_count: 1,
    };

    user1.email =  String::from("user@gmailcom");
    println!("User email changed {}", user1.email);

    //Return a struct from a function
     fn build_user(email:String, username:String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
     }

     let user2 = User{
        email: String::from("instantiate@m.com"),
        ..user1
     };

    //  Tuple Struct
     struct Colour (i32, i32, i32);
     struct Point (i32, i32, i32);

     let black:Colour = Colour(0,0,0);
     let white: Colour = Colour(255,255,255);

    //  Unit-Like Struct
    struct AlwaysEqual;
    let subject:AlwaysEqual = AlwaysEqual;
     
}