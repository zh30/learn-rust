fn main() {
	// let home = IpAddr {
	//     kind: IpAddrKind::V4,
	//     address: String::from("192.168.0.1"),
	// };
	// let loopback = IpAddr {
	//     kind: IpAddrKind::V6,
	//     address: String::from("::1"),
	// };

	let _home = IpAddrKind::V4(127, 0, 0, 1);
	let _loopback = IpAddrKind::V6(String::from("::1"));

	// println!("The home ipaddr is {}", home);
	// println!("The loopback ipaddr is {}", loopback);

	let m = Message::Write(String::from("hello"));
	m.call();
}

#[derive(Debug)]
struct IpAddr {
	kind: IpAddrKind,
	address: String,
}

struct QuitMessage;
struct MoveMessage {
	x: i32,
	y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

#[derive(Debug)]
enum IpAddrKind {
	V4(u8, u8, u8, u8),
	V6(String),
}

enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		println!("The message");
	}
}

fn route(ip_type: IpAddrKind) {}
