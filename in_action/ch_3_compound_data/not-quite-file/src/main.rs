//! Simulating files one step at a time

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Represents a "file"
#[derive(Debug)]                        // Enables file to work with printlnl! and its fmt! siblings       
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }
    /// Reads the file into memory
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> { //return number of bytes read
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();                   //make a copy of the data here
        let read_length = tmp.len();
        save_to.reserve(read_length);                   //not necessary but usefulto know about. Ensures there is sufficient space to fit theincoming data
        save_to.append(&mut tmp);                       // allocate sufficent data in the save_to buffer to hold the contents of f
        Ok(read_length)
    }
    /// Returns the file's length in bytes. 
    pub fn len(&self) -> usize {
        self.data.len() 
    }
    /// Returns the file's name. 
    pub fn name(&self) -> String {
        self.name.clone() 
    }

}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}


fn main() {
    let mut f5 = File::new("5.txt");

    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working"); 
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);    //converts Vec<u8> to String

    println!("{:?}", f5);
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);
}
