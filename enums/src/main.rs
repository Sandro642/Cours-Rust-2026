enum ipAddrKind {
    V4,
    V6,
}

enum ipAddrSring {
    V4(String),
    V6(String),
}

enum ipAddrU8 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct ipAddr {
    kind: ipAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
} // Same Thing under

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // Tuple
struct ChangeColorMessage(i32, i32, i32); // Tuple

impl Message {
    fn call(&self) {
        // Method body
    }
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let four = ipAddrKind::V4;
    let six = ipAddrKind::V6;

    route(ipAddrKind::V4);
    route(ipAddrKind::V6);

    let home = ipAddr {
        kind: ipAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = ipAddr {
        kind: ipAddrKind::V6,
        address: String::from("::1",)
    };

    let home = ipAddrSring::V4(String::from("127.0.0.1"));
    let loopback = ipAddrSring::V6(String::from("::1"));

    let home = ipAddrU8::V4(127, 0, 0, 1);
    let loopback = ipAddrU8::V6(String::from("::1"));


    //

    let m = Message::Write(String::from("Hello"));

    m.call();


    // Enum Option

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = Option::None;


    // But you can't do this !!

    //let x: i8 = 5;
    //let y: Option<i8> = Some(5);

    //let sum = x + y;
}

fn route(ip_kind: ipAddrKind) {}


//

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
