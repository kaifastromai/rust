//Structs are used to create custom data-types

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
//Tuple struct
struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    fn fullname(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    
    }
    //name to tuple

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
struct TColor(u8, u8, u8);
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;

    println!("Colors: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TColor(255, 0, 0);
    println!("Tuple Color: {} {} {}", tc.0, tc.1, tc.2);
    tc.0 = 200;

    let mut p = Person::new("John", "Doe");
    p.set_last_name("Richards");
    println!("Person {}", p.fullname());
    println!("Person {:?}", p.to_tuple());
}
