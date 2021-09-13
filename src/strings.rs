
pub fn run() {
    // Instantiate a String
    // Instantiate a str
    
    // Use String instead of &str
    let mut hello: String = String::from("hello");
    // Get length
    println!("length : {}", hello.len());

    // Push char
    hello.push('a');

    // Push string
    hello.push_str(" world");

    // Get capacity in bytes
    println!("capacity is {}", hello.capacity());

    // Check if empty
    println!("is empty : {}", hello.is_empty());

    // Check if contains
    println!("contains or? {}", hello.contains("or"));

    // Replace
    println!("replace: {}", hello.replace("or", "DUDE"));

    // Loop through String
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Set String capacity
    let mut s: String = String::with_capacity(10);

    s.push('a');
    s.push('a');

    println!("{}", s);

    // Assertion
    assert_eq!(s.len(), 2);
    assert_eq!(10, s.capacity());


}