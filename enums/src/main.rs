
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_method(&self) {
        // code here
    }
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}


fn main() {
    let localhost = IpAddrKind::V4(127,0,0,1);
}

fn route(ip_kind: IpAddrKind) {
    // code here
}
