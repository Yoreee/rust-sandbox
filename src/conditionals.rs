pub fn run() {
    let age: u8 = 18;
    let checked_id: bool = false;
    let knows_person_of_age = true;
    // if else
    if age > 21 && checked_id || knows_person_of_age {
        println!("bartenders says 'what would you like to drink?'");
    } else if age < 21 && checked_id {
        println!("youre not old enough to drink");
    } else {
        println!("can i see your id?")

    }


    // shorthand if

    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age = {}", is_of_age);
}