use jb::net::SocketAddress;

fn main() {
    let addr: SocketAddress = SocketAddress::ipv4_from("www.google.nl", 443).unwrap();
    println!("{}", addr.hostname());
}