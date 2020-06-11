//Resisable arraus

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    //Reassign vals
    numbers[2] = 20;

    //Get value
    println!("Single value: {}", numbers[0]);
    //Get length
    println!("Vector length: {}", numbers.len());

    //Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    //Add to vector
    numbers.push(6);
    println!("{:?}", numbers);

    for x in numbers.iter() {
        println!("Val: {}", x);
    }

    //Loops and mutate values
    for x in numbers.iter() {
        *X *= 2;
        println!("{}", x);
    }
}
