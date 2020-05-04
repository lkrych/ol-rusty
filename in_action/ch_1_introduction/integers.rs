use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() { 
    let a = 10;                                 // integer on stack
    let b = Box::new(20);                       // integer on heap (boxed integer)
    let c = Rc::new(Box::new(30));              // boxed integer wrapped with a reference counter
    let d = Arc::new(Mutex::new(40));           // integer protected by mutex lock wrapped with reference counter

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d); 
}