use std::fs;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
extern crate serde;

const PATH: &str = "/home/kfriedt/code/rust-serde/";

#[derive(Serialize, Deserialize, Debug)]
struct RawRegisterMap {
    device_type: String,
    manufacturer: String,
    model: String,
    registers: Vec<Register>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Register {
    address: String,
    details: RegisterDetails,
}

#[derive(Serialize, Deserialize, Debug)]
struct RegisterDetails {
    description: String,
    exp: u16,
    frequency: u16,
    name: String,
    size: u16,
    data_type: String,
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
    println!("Data: {}", data);

    // Parse the string of data into serde_json::
    let v: Value = serde_json::from_str(data)?;
    println!("The value is v: {}", v);
    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    println!("The age of John:{}", v["age"]);

    // get an int from the JSON code
    let age= v["age"].as_i64().unwrap();

    Ok(())

}

fn serialize_primitive(){
    let x: i32 = 5;
    let xs = serde_json::to_string(&x).unwrap();
    println!("i32 number {} serializes into string {}", x, xs);
    let xd: i32 = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);
}

fn register_example() {
    let filename = "src/sungrow-battery.json";
    let filepath: String = PATH.to_owned() + filename; // could use .to_owned() or .to_string() NOT .clone()


    // checked to see if variables were still available
    // println!("{}", filename);
    // println!("{}", PATH);
    // println!("{}", filepath);
//*************************************************************
    // // THIS WORKED WITH EITHER VERSION BELOW FO THE let reg_map BUT GOT A BUNCH OF OBJECTS
    // // COULDN'T ACCESS THE OBJECTS DIRECTLY
    // // With a string that is read from a file
    // let contents = fs::read_to_string(filepath).unwrap(); // could add .as_ref() to get &str
    // // println!("Contents: {}", contents);
    // let reg_map: serde_json::Value = serde_json::from_str(&contents)?; //if .unwrap().as_ref() then contents
    // let reg_map: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well formatted"); //if .unwrap().as_ref() then contents

    // // println!("Device Type {}", v["device_type"]);
    // // println!("registers {}", v["registers"]);
    // // println!("first register {}", v["register"][0]);

//****************************************************************
    // use an open file and read from the file
    // WITH THE FORMATTED JSON FILE IT WORKS
    let file = fs::File::open(filepath).expect("File should open read only");

    // // WITH THE OLD JSON FILE IT WORKS BUT HARD TO ACCESS REGISTERS
    // let filename_2 = "src/old-battery.json";
    // let filepath_2: String = PATH.to_owned() + filename_2;
    // let file = fs::File::open(filepath_2).expect("File should open read only");


    let reader = BufReader::new(file);
    let reg_map: serde_json::Value = serde_json::from_reader(reader)
        .expect("Proper JSON");

    println!("{:?}", reg_map);

    let device_type = reg_map.get("device_type")
        .expect("File should have device_type key");
    println!("Device Type: {}", device_type);

    let reg_1 = reg_map.get("registers")
        .expect("File should have registers key");
    println!("Registers: {}", reg_1);
    println!("Register 1: {}", reg_1[0]);
    println!("Register 1: {:?}", reg_1[0].get("details"));
//*****************************************************************
    // // HAD TO RE-FORMAT THE JSON FILE TO GET THE OBJECT
    // // TRY GETTING OBJECTS
    // let contents = fs::read_to_string(filepath).unwrap(); // could add .as_ref() to get &str
    // let reg_map: RawRegisterMap = serde_json::from_str(&contents).expect("JSON was not well formatted"); //if .unwrap().as_ref() then contents
    // println!("{:?}", reg_map);
    //
    // println!("Device Type: {:?}", reg_map.device_type);
    // println!("Registers: {:?}", reg_map.registers);
    // println!("First register address: {:?}", reg_map.registers[0].address);

    // Ok(()) // have to return () to use the ? operator in the serde_json::from_str(&contents)?
}

fn main() {
    let path_filename = "/home/kfriedt/code/rust-serde/src/example.json";
    // go over opening files in rust

    // let contents = fs::read_to_string(path_filename); // Leaves this as a Result<>
    // println!("With text:\n{:?}", contents.as_ref().unwrap()); // Have to unwrap the Result<>
    // println!("Can use the path_filename again: {}", path_filename);
    // // Need to convert the contents: Result<String> to a String?
    // untyped_example(contents.as_ref().unwrap());
    // untyped_str_example();
    // serialize_primitive();
    register_example();

}
