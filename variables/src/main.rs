fn main() {
    // mutable varaibale 
    let mut x = 5;

    // constant 
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    println!("the constant value is {THREE_HOURS_IN_SECONDS}");
}
