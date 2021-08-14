#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn r#enum() {
    let ipv4 = IpAddrKind::V4;
    println!("IP V4: {:?}", ipv4);

    let home = IpAddr {
        kind: ipv4,
        address: String::from("127.0.0.1"),
    };
    println!("Home Address: {:?}", home);

    let message_a = Message::Move{x: 20, y:30};
    println!("Message move: {:?}", message_a);

    let six = Some(6);
    let seven = plus_one(six);
    println!("Option T: {:?}", seven.unwrap());
    println!("Option T: {:?}", seven.expect("Whattttt?"));

}

fn plus_one(opt: Option<i32>) -> Option<i32> {
    match opt {
        None => None,
        Some(x) => Some(x + 1),
    }
}