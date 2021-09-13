pub fn run() {

    println!("hello from print.rs");
    println!("{}", 1);
    println!("its a meee, {name}, and my friend, {friend_name}", name="mario", friend_name="luigi");
    println!("im {number} years old", number=10000);
    println!("whoa look at this list {:b}, {:x}, {:o}", 20,20,20);
    println!("{:?}", (12, "hey", true));
    println!("10+20={}", 20+10);
    println!("my name is {0}. {1} {0}", "bond", "james");
    println!("{n:0>w$}", n=10, w=30);

}