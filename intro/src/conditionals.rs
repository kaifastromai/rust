pub fn run() {
    let age = 18;
    let check_id = false;
    let knows_of_age = true;
    //Ifelse
    if age >= 21 || knows_of_age {
        println!("Bar tender what would you like to drink");
    } else if age < 21 && check_id {
        println!("I will need to check your id");
    }else{

    }

    let is_of_age=if age>=21{true} else {false};
    
}
