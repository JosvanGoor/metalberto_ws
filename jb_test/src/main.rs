use jb::time::{Time, LOG_FORMAT};

fn main() {
    let time = Time::now();

    match time.format(LOG_FORMAT) {
        Ok(formatted) => println!("{}", formatted),
        Err(err) => println!("{:?}", err),
    }

    println!("time: {}", time.format(LOG_FORMAT).unwrap());
}