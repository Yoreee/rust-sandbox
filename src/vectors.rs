use std::mem;

pub fn run() {
    // Instantiate a vector
    let mut vec: Vec<i32> = vec![10, 99, 100, 2900];
    println!("{:?}", vec);

    // Get single value
    println!("{:?}", vec[2]);

    // Get length
    println!("{:?}", vec.len());

    // Get size
    println!("{:?}", mem::size_of_val(&vec));

    // Get slice
    let slice: &[i32] = &vec[0..2];
    println!("{:?}", slice);

    // Loop through a vector
    for x in vec.iter() {
        println!("{}", x);
    }

    for x in vec.iter_mut() {
        *x += 2
    }

    println!("itermut = {:?}", vec);
}