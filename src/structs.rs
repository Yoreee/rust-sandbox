// Create custom data types

// Classic struct with named fields
struct Color<'a> {
    red: u8,
    blue: u8,
    green: u8,
    name: &'a str
}

// Tuple Struct (no named fields, access with dot index notation)
struct Animal<'a>(&'a str, i32);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Constructor
    fn new (first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn get_full_name (&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name (&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    // name to tuple
    // pass self here because we dont need to borrow the values
    fn to_tuple (&self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }

}

pub fn run() {
    let mut c = Color {
        red: 255,
        blue: 22,
        green: 93,
        name: "idk"
    };
    c.red = 50;
    println!("{},{},{}, {}", c.red, c.blue, c.green, c.name);

    // Tuple Struct
    let owl = Animal("a", 2);

    println!("{} {}", owl.0, owl.1);

    let mut p = Person::new("Jane", "Doe");
    println!("{} {}", p.first_name, p.last_name);
    p.set_last_name("Jenkins");
    println!("{}", p.get_full_name());
    println!("{:?}", p.to_tuple());


}