enum IpAddrKind {
    V4(u32, u32, u32, u32),
    V6(String)
}
fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));
}