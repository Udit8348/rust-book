/**
 * Basic syntax rules of rust.
 * Code playground if needed.
 */
use std::hint::black_box;

// const can be declared in any scope and  differs from mut since it only allows rhs to be other consts (cannot be computed at runtime) 
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 6;
    println!("the value of x is {x}");
    
    // x = 5;
    // println!("not permitted by the rust compiler because variables are immutable by default");

    println!("the number of seconds in three hours is: {THREE_HOURS_IN_SECONDS}");


    // there are scalar types
    let integer : i8 = 9;
    let truth : bool = false;
    let float : f32 = 3.14;
    let character : char = 'u';

    println!("integer: {integer} boolean: {truth} float: {float} character: {character}");
}

// mental model: reference &T is the noun, and borrow is the verb.
// you borrow a value by creating a refernece to it
fn reference_borrow_example() -> () {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
}

// shadowing lets us "reuse" a variable but give it a different type
// the compiler can choose to reuse the space on the stack (depending on optimization level)
// see compiler explorer: https://godbolt.org/#g:!((g:!((g:!((h:codeEditor,i:(filename:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,selection:(endColumn:5,endLineNumber:1,positionColumn:5,positionLineNumber:1,selectionStartColumn:5,selectionStartLineNumber:1,startColumn:5,startLineNumber:1),source:'use+std::hint::black_box%3B%0A%0A%23%5Binline(never)%5D%0Apub+fn+square(num:+i32)+-%3E+i32+%7B%0A++++num+*+num%0A%7D%0A%0A%23%5Bno_mangle%5D%0A%23%5Binline(never)%5D%0Apub+fn+shadowing_saves_space()+-%3E+()+%7B%0A++++let+spaces+%3D+%22++++++%22%3B%0A++++//+black_box(%26spaces)%3B+//+forces+materialization%0A++++%0A++++let+spaces+%3D+spaces.len()%3B%0A++++black_box(%26spaces)%3B+//+prevents+folding+and+register-only+optimization%0A%7D%0A'),l:'5',n:'0',o:'Rust+source+%231',t:'0')),k:50,l:'4',n:'0',o:'',s:0,t:'0'),(g:!((h:compiler,i:(compiler:r1900,filters:(b:'0',binary:'1',binaryObject:'1',commentOnly:'0',debugCalls:'1',demangle:'0',directives:'0',execute:'1',intel:'0',libraryCode:'0',trim:'1',verboseDemangling:'0'),flagsViewOpen:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,libs:!(),options:'-C+opt-level%3D3',overrides:!(),selection:(endColumn:12,endLineNumber:11,positionColumn:12,positionLineNumber:11,selectionStartColumn:12,selectionStartLineNumber:11,startColumn:12,startLineNumber:11),source:1),l:'5',n:'0',o:'+rustc+1.90.0+(Editor+%231)',t:'0')),k:50,l:'4',n:'0',o:'',s:0,t:'0')),l:'2',n:'0',o:'',t:'0')),version:4
pub fn shadowing_saves_space() -> () {
    let spaces = "      ";
    // black_box(&spaces);
    
    let spaces = spaces.len();
    black_box(&spaces); // prevents folding and register-only optimization
}