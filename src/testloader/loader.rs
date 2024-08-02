use std::{fs::File};
use std::io::Read;
use serde_json::Value;

use crate::structure::{test_structure::TestQAC, test_structure_adv::TestSet};

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_json_probe(filecontent: &str) -> Result<i8,std::io::Error> {
    let result: Value = serde_json::from_str(filecontent).unwrap();
    if result.get("metadata").is_some() {
        Ok(1)
    } else {
        Ok(0)
    }
}

pub fn parse_json(filecontent: &str) -> Result<Vec<TestQAC>, std::io::Error>{
    println!("Parse json content");
    let result: Vec<TestQAC> = serde_json::from_str(filecontent).unwrap();
    Ok(result)
}

pub fn parse_json_adv(filecontent: &str) -> Result<TestSet, std::io::Error>{
    println!("Parse json content");
    let result = serde_json::from_str(filecontent).unwrap();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    static CONTENT_TESTSTRUCTURE_V1: Lazy<String> = Lazy::new(|| format!("{}"," 
    [
        {
            \"que\": \"What is SSH port number\",
            \"ans\": [
                \"25\",
                \"443\",
                \"22\",
                \"69\"
            ],
            \"correct\": [
                false,
                false,
                true,
                false
            ]
        },
        {
        \"que\": \"What is FTP port number\",
        \"ans\": [
            \"44\",
            \"1025\",
            \"20\",
            \"21\"
        ],
        \"correct\": [
            false,
            false,
            true,
            true
        ]
    }
]   "));
static CONTENT_TESTSTRUCTURE_V2: Lazy<String> = Lazy::new(|| format!("{}","
    {\"metadata\": 
        {
        \"version\" : \"0.2\",
        \"author\" : \"Wolf\"
        },
     \"config\" : 
        {
        \"rand_questions\": true,
        \"rand_answers\": true,
        \"rand_max_answers\" : 4,
        \"rand_min_answers\" : 6,
        \"rand_set_answers\" : [\"21\",\"22\",\"23\",\"25\",\"67\",\"110\",\"138\",\"548\",\"3389\"]
        },
    \"content\": [ 
        {
            \"que\": \"What is SSH port number\",
            \"ans\": [\"22\"]
        },
        {
            \"que\": \"What is FTP port number\",
            \"ans\": [\"20\",\"21\"]
        }
        ] 
    }  
"));


    #[test]
    fn test_readfile() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        
        temp_file
        .write_all(CONTENT_TESTSTRUCTURE_V1.as_bytes())
        .expect("Failed to write to temp file");
        let file_path = temp_file.path().to_str().unwrap();
        let result = read_file(file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), *CONTENT_TESTSTRUCTURE_V1);
    }

    #[test]
    fn test_parse_json() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
        .write_all(CONTENT_TESTSTRUCTURE_V1.as_bytes())
        .expect("Failed to write to temp file");
        let file_path = temp_file.path().to_str().unwrap();
        let parsed = read_file(&file_path).expect("Error reading file");
        let test_content = parse_json(&parsed).unwrap();
        let testlength = test_content.len();
        assert_eq!(testlength,2); 
    }
    
    #[test]
    fn test_readfile_v2() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        
        temp_file
        .write_all(CONTENT_TESTSTRUCTURE_V2.as_bytes())
        .expect("Failed to write to temp file");
        let file_path = temp_file.path().to_str().unwrap();
        let result = read_file(file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), *CONTENT_TESTSTRUCTURE_V2);
    }

    #[test]
    fn test_parse_json_v2() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
        .write_all(CONTENT_TESTSTRUCTURE_V2.as_bytes())
        .expect("Failed to write to temp file");
        let file_path = temp_file.path().to_str().unwrap();
        let parsed = read_file(&file_path).expect("Error reading file");
        let test_content_v2 = parse_json_adv(&parsed).unwrap();
        let testcontent = test_content_v2.get_content().clone();
        assert_eq!(testcontent.len(),2); 
    }
}