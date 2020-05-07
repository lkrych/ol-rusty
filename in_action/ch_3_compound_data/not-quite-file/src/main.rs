#![allow(unused_variables)]             //Silences the warnings caused by open and close

#[derive(Debug)]                        // Enables file to work with printlnl! and its fmt! siblings       
struct File {
    name: String,
    data: Vec<u8>,                      //Using Vec<u8> provides access to some useful conveniences such as writing to a file
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize { //return number of bytes read
    let mut tmp = f.data.clone();                   //make a copy of the data here
    let read_length = tmp.len();
    save_to.reserve(read_length);                   //not necessary but usefulto know about. Ensures there is sufficient space to fit theincoming data
    save_to.append(&mut tmp);                       // allocate sufficent data in the save_to buffer to hold the contents of f
    return read_length;
}


fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);    //converts Vec<u8> to String

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, &f2_length);
    println!("{}", text);
}
