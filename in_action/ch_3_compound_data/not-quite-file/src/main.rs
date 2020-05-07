#![allow(unused_variables)]             //Silences the warnings caused by open and close

#[derive(Debug)]                        // Enables file to work with printlnl! and its fmt! siblings       
struct File {
    name: String,
    data: Vec<u8>,                      //Using Vec<u8> provides access to some useful conveniences such as writing to a file
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        return f;
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize { //return number of bytes read
        let mut tmp = self.data.clone();                   //make a copy of the data here
        let read_length = tmp.len();
        save_to.reserve(read_length);                   //not necessary but usefulto know about. Ensures there is sufficient space to fit theincoming data
        save_to.append(&mut tmp);                       // allocate sufficent data in the save_to buffer to hold the contents of f
        return read_length;
    }
    
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}


fn main() {
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("2.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    open(&mut f3);
    let f3_length = &f3.read(&mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);    //converts Vec<u8> to String

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text);
}
