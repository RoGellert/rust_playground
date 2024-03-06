use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = match File::create("foo.txt") {
        Ok(file) => file,
        Err(error) => panic!("Can't create the file. Error: {:?}", error),
    };
    let _ = match file.write_all(b"Hello, world!"){
        Ok(()) => {},
        Err(error) => panic!("Can't write to the file: {:?}", error),
    };

    let _ = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => panic!("Can't open the file Error: {:?}", error),
    };
}
