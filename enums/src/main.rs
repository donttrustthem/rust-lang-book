fn main() {
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    //
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // enum IpAddrKind {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }
    //
    // let home = IpAddrKind::V4(127, 0, 0, 1);
    // let loopback = IpAddrKind::V6(String::from("::1"));
    //
    // struct Ipv4Addr {
    //
    // }
    // struct Ipv6Addr {
    //
    // }
    //
    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }

    enum Message {
        // Quit,
        // Move { x: i32, y: i32},
        Write(String),
        // ChangeColor(i32, i32, i32),
    }

    // struct QuitMessage;
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String);
    // struct ChangeColorMessage(i32, i32, i32);

    impl Message {
        fn call(&self) {

        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}

// fn route (ip_kind: ipAddrKind) {}
