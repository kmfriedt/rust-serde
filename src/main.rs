use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
struct RawRegisterMap {
    device_type: String,
    manufacturer: String,
    model: String,
    registers: Vec<Register>,
}

struct Register {
    description: String,
    exp: u16,
    frequency: u16,
    name: String,
    size: u16,
    data_type: u32,
    value: i32,
    tags: Vec<String>,
    writeable: bool,
}



fn untyped_example(data: &String) -> Result<()> {
    let v: Value = serde_json::from_str(&data)?;
    println!("Untyped example string from file");
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    Ok(())
}

fn untyped_str_example() -> Result<()> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    println!("The age of John:{}", v["age"]);

    let age= v["age"].as_str();
    Ok(())

}

fn serialize_primitive(){
    let x: i32 = 5;
    let xs = serde_json::to_string(&x).unwrap();
    println!("i32 number {} serializes into string {}", x, xs);
    let xd: i32 = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);
}

fn main() {
    let path_filename = "/home/kfriedt/code/rust-serde/src/example.json";
    let contents = fs::read_to_string(path_filename);
    println!("With text:\n{:?}", contents.as_ref().unwrap());

    // Need to convert the contents: Result<String> to a String?
    untyped_example(contents.as_ref().unwrap());
    untyped_str_example();
    serialize_primitive();

}
