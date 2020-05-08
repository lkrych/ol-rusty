static mut ERROR: i32 = 0; // a global variable that is valid for the life of the program

// ...

fn main() {
    //...
    read(f, buffer);

    unsafe {        //accessing and modifying static mut variables requires using the unsafe block. This is Rut's way of disclaiming resposibility
        if ERROR != 0 {
            panic!("An error has occurred")
        }
    }
}