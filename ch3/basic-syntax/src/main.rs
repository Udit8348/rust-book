/**
 * Basic syntax rules of rust.
 * Code playground if needed.
 */

// const can be declated in any scope and  differs from mut since it only allows rhs to be other consts (cannot be computed at runtime) 
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 6;
    println!("the value of x is {x}");
    
    // x = 5;
    // println!("not permitted by the rust compiler because variables are immutable by default");

    println!("the number of seconds in three hours is: {THREE_HOURS_IN_SECONDS}");
}
