use std::char;

fn main() {
    let str = "C##";
    str.trim();
    assert_eq!(true, str.is_ascii());
    
    let v: Vec<char> = str.chars().collect();
    println!("{:?}", v);
}
