pub fn run() {
    //Primnative array

    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("Values: {:?}", (arr1, arr2));

    //Non primatives
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values {:?}", (&vec1, vec2));
}
