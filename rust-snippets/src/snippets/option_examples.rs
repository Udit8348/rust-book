/**
 * Option has two variants:
 * 
 * enum Option<T> {
 *    Some(T),
 *    None,
 * }
 * 
 * We can use the value of Option<T> any of the following:
 * - match
 * - if let
 * - map and then
 * - unwrap_or
 * 
 */

pub fn run() {
    basic_some_none();
    match_example();
    if_let_example();
    map_and_then_example();
    unwrap_or_example();
}

fn basic_some_none() {
    let x: Option<i32> = Some(10);
    let y: Option<i32> = None;

    // "x = Some(10), y = None"
    println!("x = {:?}, y = {:?}", x, y);

    // "x_value = 10"
    let x_value = x.unwrap_or(0);
    println!("x_value = {}", x_value);
}

fn match_example() {
    let value = Some(5);

    // "Got value: 5"
    match value {
        Some(v) => println!("Got value: {}", v),
        None => println!("No value"),
    }
}

fn if_let_example() {
    let value = Some(42);

    // "if let extracted 42"
    if let Some(v) = value {
        println!("if let extracted {}", v);
    }
}

fn map_and_then_example() {
    let value = Some(3);

    // "squared = Some(9)"
    let squared = value.map(|v| v * v);
    println!("squared = {:?}", squared);
    
    // "chained = Some(8)"
    let chained = value
        .and_then(|v| Some(v + 1))
        .map(|v| v * 2);

    println!("chained = {:?}", chained);
}

fn unwrap_or_example() {
    let none_value: Option<i32> = None;

    // "defaulted = 100"
    let defaulted = none_value.unwrap_or(100);
    println!("defaulted = {}", defaulted);
}