use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();           // file requires a path argument and error handling
    let mut reader = BufReader::new(f);

    let mut line = String::new();                       // Reuse a String object over the lifetime of the program
    loop {                                              // loops until the program encounters return, break or panics
        let len = reader.read_line(&mut line).unwrap(); // reading from disk can fail and we need to explicitly handle it
        if len == 0 {
            break
        }

        println!("{} ({} bytes long)", line, len);

        line.truncate(0);                               // shrink the String back to length 0, preventing lines from persisting into the following ones
    }
}

// manually looping through a file can be cumbersome
// for the common cases, iterating through lines is more commonly used

// fn option() {
//     let f = File::open("readme.md").unwrap();           // file requires a path argument and error handling
//     let mut reader = BufReader::new(f);

//     for line_ in reader.lines() {
//         let line = line_.unwrap();
//         println!("{} ({} bytes long)", line, line.len());
//     }
// }