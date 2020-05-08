extern crate rand;                      //bring rand crate into scope
use rand::Rng;                          //bring random number generator trait into scope

fn one_in(n: u32) -> bool {
    return rand::thread_rng().gen_weighted_bool(n);    //helper function to trigger sporadic errors
}

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

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> { //return number of bytes read
        let mut tmp = self.data.clone();                   //make a copy of the data here
        let read_length = tmp.len();
        save_to.reserve(read_length);                   //not necessary but usefulto know about. Ensures there is sufficient space to fit theincoming data
        save_to.append(&mut tmp);                       // allocate sufficent data in the save_to buffer to hold the contents of f
        Ok(read_length)
    }

    
}
fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal");
        return Err(err_msg);
    }
    Ok(f)
}


fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);    //converts Vec<u8> to String

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
