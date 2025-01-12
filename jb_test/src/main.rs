use jb::time::{Time, LOG_FORMAT};

fn main() {
    let time = Time::now();
    println!("time: {}", time.format(LOG_FORMAT).unwrap());
}