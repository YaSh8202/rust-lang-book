use core::panic;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn read_username_from_file()->Result<String, io::Error>{
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)

    


}


fn main() {
    let f = File::open("hello.txt").expect("Failed to open hell.txt");

    

}
