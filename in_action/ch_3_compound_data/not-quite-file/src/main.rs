#[derive(Debug)]                        // Allows the println! macro to print File         

struct File {
    name: String,
    data: Vec<u8>,                      //Using Vec<u8> provides access to some useful conveniences such as writing to a file
}


fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
