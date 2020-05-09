#![allow(unused_variables)] //silence any warnings to unused variables

#[derive(Debug)]
struct File;

trait Read {    //a trait block includes the type signatures of functions that implementors must comply with
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File { //impl block attaches a trait to a type
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}
