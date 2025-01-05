use std::fs::OpenOptions;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

/*
Approaches to open and read file:-
1. Using read():-> Reads entire file as vector
2. read_string()
3. BufReader ->can read line by line with help of => lines()

Write:
write_all()
Append to file=> Use OpenOptions enum
.read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("foo.txt");
*/

fn read_by_vector(name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(name)?;
    let mut buf = vec![];
    let _ = file.read_to_end(&mut buf);

    Ok(buf)
}

fn read_by_string(name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(name)?;
    let mut res = String::new();
    let _ = file.read_to_string(&mut res);
    Ok(res)
}

fn read_by_bufreader(name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(name)?;
    let mut buf_reader = BufReader::new(file);
    let mut res = String::new();
    let _ = buf_reader.read_to_string(&mut res);
    Ok(res)
}

fn write_by_clearing(name: &str, content: &str) {
    let mut file = File::create("samba.txt").expect("error creating file");
    file.write_all(content.as_bytes());
}

fn write_by_appendings(name: &str, content: &str) {
    let mut file = OpenOptions::new().read(true).write(true).append(true).open(name).expect("Error file not found");
    file.write_all(content.as_bytes());

}

fn main() {
    println!("Welcome to file handling");

    let data = read_by_vector("./working_with_text_file/testFile.txt").unwrap();
    println!("{:?}", data);

    println!("Printing the content using read_to_string");

    println!("{}", read_by_string("./working_with_text_file/testFile.txt").unwrap());

    println!("Printing the content using Bufreader");

    println!("{}", read_by_bufreader("./working_with_text_file/testFile.txt").unwrap());

    println!("Writting the content to a file");
    write_by_clearing("./working_with_text_file/samba.txt", "samba file sa sa sa samba file");
    println!("Writting the content to a file");
    write_by_appendings("./working_with_text_file/samba.txt", "sidddd samba file sa sa sa samba file");
}
	