extern crate serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// This function reads a JSON file named "data.json" and deserializes its content
/// into a HashMap using `serde_json`. It demonstrates two methods of deserialization:
/// the typical method and using the Turbofish syntax. The function then prints the
/// value associated with the key "cat" from the deserialized HashMap and asserts
/// that both deserialization methods produce equivalent results.

fn deserialize_to_hashmap() {
    let filename = "./working_with_json_file/data.json";
    let content = std::fs::read_to_string(filename).unwrap();

    let leggs: HashMap<String, u32> = serde_json::from_str(&content).unwrap();
    println!("leggs:     {:?}", leggs["cat"]);
    // doing the same using Turbofish
    let turbofish = serde_json::from_str::<HashMap<String, u32>>(&content).unwrap();
    println!("turbofish: {turbofish:?}");

    assert_eq!(leggs, turbofish);
}

/// This function manually serializes a hashmap to a JSON string, deserializes
/// this string back into a hashmap and checks if the data is correct.
///
/// This example shows how to manually serialize a hashmap to a JSON string and
/// read it back into a hashmap.
fn serialize_hashmap() {
    let mut data_before = HashMap::new();
    data_before.insert(String::from("foo"), 23);
    data_before.insert(String::from("bar"), 42);
    println!("data_before:  {data_before:?}");

    let json_string = serde_json::to_string(&data_before).unwrap();
    println!("serialized: {}", json_string);

    let data_after: HashMap<String, u32> = serde_json::from_str(&json_string).unwrap();
    println!("data_after:  {data_after:?}");

    println!(
        "can i add a valur which is deserialized: a + b = {}",
        data_after["foo"] + data_after["bar"]
    );

    let data_turbofish = serde_json::from_str::<HashMap<String, u32>>(&json_string).unwrap();
    println!("turbofish:    {data_turbofish:?}");
    assert_eq!(data_before, data_turbofish);
}

/// This function reads a simple JSON file and manually deserializes it.
///
/// This example shows how to manually read a JSON file and
/// extract values from it.
fn read_manually_json() {
    //read simple json manually

    let filename = "./working_with_json_file/data2.json";
    let content = std::fs::read_to_string(filename).unwrap();
    let data: serde_json::Value = serde_json::from_str(&content).expect("Parsing Error JSON");
    println!("data: {data:?}");

    if data.is_object() {
        for key in data.as_object().unwrap().keys() {
            println!("key: {key}");
        }
    }

    println!();

    match data.get("text") {
        None => println!("key not found"),
        Some(text) => {
            println!("this is text: {}", text.is_string());
            println!("this is text: {}", text.is_u64());
        }
    }

    let text = match data.get("text") {
        None => panic!("Field text does not exist"),
        Some(val) => val.as_str().unwrap(),
    };
    println!("text: {text}");

    let x = match data.get("x") {
        Some(val) => val.as_i64().unwrap(),
        None => panic!("Field x does not exist"),
    };
    println!("x: {x}");
    let y = match data.get("y") {
        Some(val) => val.as_i64().unwrap(),
        None => panic!("Field y does not exist"),
    };
    println!("y: {y}");

    println!("x+y = {}", x + y);

    let f = match data.get("f") {
        Some(val) => val.as_f64().unwrap(),
        None => panic!("Field y does not exist"),
    };
    println!("f: {f}");
}

/// Read a JSON file into a struct using `from_reader`.
///
/// Example: person.json
///
fn read_to_struct_using_from_reader() {
    #[derive(Deserialize, Debug)]
    struct Person {
        fname: String,
        lname: String,
        married: bool,
    }

    let filename = "./working_with_json_file/person.json";
    let content = std::fs::read_to_string(filename).unwrap();
    let person: Person = serde_json::from_str(&content).unwrap();
    println!("person: {person:?}");

    //OR

    let data: Person = match std::fs::File::open(&filename) {
        Ok(file) => serde_json::from_reader(&file).expect("JSON parsing error"),
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        }
    };
    println!("{:#?}", &data);
    assert_eq!(data.fname, "Foo");
    assert_eq!(data.lname, "Bar");
    assert!(data.married);
}

/// Demonstrates how to handle missing fields in a JSON object during deserialization.
///
/// This example shows how to specify default values for missing fields in a JSON object.
/// The `#[serde(default = "...")]` attribute is used to specify a default value for a field.
/// The default value can be a function that returns a default value.
///
/// In this example, the `fname` field is given a default value of `"Foo"` and the `married`
/// field is given a default value of `false` if they are missing from the JSON object.
///
/// The `#[allow(dead_code)]` attribute is used to suppress warnings about unused fields.
fn handle_missing_fields() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Person {
        #[serde(default = "get_default_fname")]
        fname: String,

        #[serde(default = "get_default_false")]
        married: bool,
    }

    fn get_default_fname() -> String {
        String::from("Foo")
    }

    fn get_default_false() -> bool {
        false
    }

    let content = "{}";
    let data = serde_json::from_str::<Person>(content).expect("JSON parsing error");
    println!("{:#?}", &data);
}

fn optional_fields(){
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Person {
        fname: String,

        #[serde(default = "get_default_language")]
        language: String,

        married: Option<bool>,
    }

    fn get_default_language() -> String {
        String::from("Rust")
    }

    let filename = "./working_with_json_file/person.json";

    let content = std::fs::read_to_string(filename).unwrap();

    let data = serde_json::from_str::<Person>(&content).expect("JSON parsing error");
    println!("{:#?}", data);

    match data.married {
        None => println!("We don't know if {} is married or not", data.fname),
        Some(val) => println!("Marrige status: {val}"),
    }
}
fn main() {
    #[derive(Serialize, Debug)]
    struct Person {
        fname: String,
        lname: String,
        married: bool,
    }

    let p1 = Person {
        fname: String::from("Foo"),
        lname: String::from("Bar"),
        married: true,
    };
    let json_string = serde_json::to_string(&p1).unwrap();
    println!("json Person: {}", json_string);
}
