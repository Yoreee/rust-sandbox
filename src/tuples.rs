// Use tuples as arguments and return values
fn reverse_tuple(pair: (&str, i8)) -> (i8, &str) {
    let (string, integer) = pair;
    (integer, string)
}

pub fn run() {
    // Use function above
    println!("{:?}", reverse_tuple(("hey", 90)));

    // Create a tuple of different types
    let person: (&str, &str, i8) = ("jack", "mack", 60);

    // Extract values from tuple using their index
    println!("{} is from {} and is age {}", person.0, person.1, person.2);

    //

}