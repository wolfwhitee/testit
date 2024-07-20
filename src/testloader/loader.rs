use std::fs::File;
use std::io::Read;
use crate::structure::test_structure::TestQAC;


pub fn showinfo() {
    print!("Test loader [loaded!]")
}

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_json(content: &str) -> Result<Vec<TestQAC>, std::io::Error>{
    println!("Parse json content");
    let result: Vec<TestQAC> = serde_json::from_str(content).unwrap();
    Ok(result)
}