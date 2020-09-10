fn main() {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("192.168.0.1"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("The home ipaddr is {}", home);
    println!("The loopback ipaddr is {}", loopback);
}

// #[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(ip_type: IpAddrKind) {}
