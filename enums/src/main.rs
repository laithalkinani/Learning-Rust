enum IpAddrKind {
    V4(String),
    V6(String),
}


fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let five = IpAddrKind::V5(String::from("::1"));
}
