
pub fn run() {

    let name = "re";
    let mut age = 20;
    age = 21;
    println!("age: {}, name: {}", age, name);

    const ID: i32 = 20;
    println!("{}", ID);

    //i32
    let x = 1; 

    //f64
    let y = 2.5; 

    let z: i64 = 454545454;

    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    let is_active = true;
    let is_greater = 10 < 1;
    let a1: char = 'v';
    let face = '\u{1F607}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

}