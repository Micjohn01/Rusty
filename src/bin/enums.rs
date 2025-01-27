fn main() {
    enum IpAddrKind {
        V4,
        V6
    }

let _four:IpAddrKind = IpAddrKind::V4;
let _six:IpAddrKind = IpAddrKind::V6;

fn route(_ip_kind: IpAddrKind){}

route(IpAddrKind::V4);
route(IpAddrKind::V6);

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Using Enums
// let _home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
// let _loopback: IpAddr = IpAddr::V6(String::from("::1"));

// Enhance Enums
let _home: IpAddr = IpAddr::V4(127,0,0,1);
let _loopback: IpAddr = IpAddr::V6(String::from("::1"));

// Using Structs

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

// let home: IpAddr = IpAddr{
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let home2: IpAddr = IpAddr{
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

}