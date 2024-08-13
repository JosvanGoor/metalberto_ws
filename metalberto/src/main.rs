use jb::{hash::Sha1, net::SocketAddress};

fn main() {
    let mut sha1 = Sha1::new();
    sha1.update("The quick brown fox jumped over the lazy fox".as_bytes());
    println!("empty hash: {}", sha1.finalize_as_str());
}