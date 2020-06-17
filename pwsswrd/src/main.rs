use std::env;
extern crate chrono;
use chrono::{Datelike, Timelike};
fn main() {
    println!("Specify using name, class, version");
    let name = env::args().nth(1).unwrap();
    let class = env::args().nth(2).unwrap();
    let version = env::args().nth(3).unwrap();

    println!(
        "name?={}&&Class?={}&&Date?={}&&v?={}",
        name,
        class,
        chrono::Local::now().to_rfc3339(),
        version
    )
}
