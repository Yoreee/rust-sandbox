pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count+=1;
        println!("{}", count);

        if count > 20 {
            break;
        }
    }


    let mut countfb = 0;
    // Fizz Buzz
    while countfb <= 100 {
        if countfb % 15 == 0 {
            println!("fizzbuzz");
        } else if countfb % 3 == 0 {
            println!("fizz");
        } else if countfb % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", countfb);
        }

        countfb +=1
    }

    
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}