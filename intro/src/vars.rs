//Variables hold primitive data or references to data
//Variables are immutbale by default
//Rust is a block-scoped
pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 36;
    println!("My name is {} and I am {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);
    //Multiple assign

    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
    
}
