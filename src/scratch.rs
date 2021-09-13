#![allow(unused)]
pub fn run() {
    println!("he");
    let one = 1;
    let one_ref = &one;
    let one_deref = *one_ref;

    println!("one {}, one_ref {} deref {}", one, one_ref, one_deref);
    
}