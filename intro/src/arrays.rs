//Arrays - FIxed list where elements are the same data types
use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    //Reassign vals
    numbers[2] = 20;

    //Get value
    println!("Single value: {}", numbers[0]);
    //Get length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}
