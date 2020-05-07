#![allow(unused_variables)]             //relax compiler warnings

type File = String;                     //create type alias

fn open(f: &mut File) -> bool {
    true                                //assume these functions succeeds
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]                                         //relaxes compiler warnings
fn read(f: &mut File, save_to: &mut Vec<[u8]>) -> ! {       //Using ! as return type indicates to the Compiler that this function never returns
    unimplemented!()                                        // A macro that crashes the program if it is encountered
}

fn main() {
    let mut f1 = File::from("f1.txt");                      //With the type declaration on line 3, File inherits String methods
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}
