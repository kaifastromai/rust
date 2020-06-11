//Primitive str=immutable fixed-length string somewhere in memory
//String=growable,heap-allocated data structure
pub fn run() {
    let mut hello = String::from("hello");
    println!("Length: {}", hello.len());
    hello.push_str("world");

    //Loop through string

    for word in hello.split_whitespace() {
        println!("Word {} \n", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    //Assertion testing
    assert_eq!(3, s.len());
}
