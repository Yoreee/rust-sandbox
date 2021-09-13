pub fn run() {
    greeting("yoooo my g", "brad");

    let get_sum = add(10, 42);
    println!("{}", get_sum);

    // Closure
    let n3 = 4;
    let add_sums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("{}", add_sums(1,2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} hell", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}








