use std::mem;

pub fn run() {
    // Instantiate array of fixed length
    let arr: [i32; 4] = [1,2,3,4];

    // Show array
    println!("{:?}", arr);

    // Select an item from array using index and bracket notation
    println!("{}", arr[0]);

    // Get length
    println!("Length {}", arr.len());
    
    // Stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&arr));

    // Get slice
    let slice: &[i32] = &arr[1..3];
    println!("slice : {:?}", slice);
}