//Prim types
//: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,
//: Floats: f32,f64
//: Boolean
//: Characters (char)
//: Tuples
//: Arrays

pub fn run() {
    //Default is i32
    let x = 1;
    //Default is f64
    let y = 2.5;
    //explicit type
    let k: i64 = 456767;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //booleans
    let b_is_active = true;
    println!("{:?}", (x, y, k, b_is_active));
}
