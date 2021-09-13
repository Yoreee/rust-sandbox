pub fn run() {
    // primitive array
    let arr1 = [10, 20, 500];
    let arr2 = arr1;
    println!("{:?}", arr2);

    // vector

    let vec1: Vec<i32> = vec![6,7,33,92];
    let vec2 = &vec1;
    println!("{:?}", (vec1));
}