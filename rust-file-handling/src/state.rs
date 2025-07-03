use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::serde;

pub fn read_file(file_name: &str) -> Map<String, Value>{
    let mut file = File::open(
        file_name.to_string()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
}