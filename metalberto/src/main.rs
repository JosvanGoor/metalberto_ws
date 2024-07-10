use jb::SocketAddress;

fn main() {
    let mut addr = SocketAddress::new();    
    addr.set_host("farts").unwrap();
    println!("hostname: {}", addr.hostname());
}