use std::io;
use std::env::{self};
use crate::structure::args_structure::ArgsS;

pub fn parse() -> Result<ArgsS,std::io::Error> {
    //let filename = "D:/dev/rust/testit/src/example/test.json";
    //let filename = "D:/dev/rust/testit/src/example/test_ports.json";
    let filename = "D:/dev/rust/testit/src/example/test_ports_v0.2.json".to_string();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut args_instance = ArgsS::new(String::from(filename));
    for arg in args {
        println!("{:?}",arg.to_string());
        let var_name = arg.clone();
        let key_value = split_arg(&var_name).unwrap().clone();
        if key_value.get(0).unwrap().clone()=="filepath".to_string() {
            args_instance.set_filepath(String::from(key_value.get(1).unwrap().clone()));        
        }
    }
    Ok(args_instance)
}

pub fn split_arg(key_value: &str) -> Result<Vec<String>,io::Error> {
    println!("Key value: {}",key_value);
    if  key_value.contains("=") {
        let (key, value) = key_value.split_once('=').unwrap_or(("unknown", ""));
        let _key_value: Vec<String> = vec![key.to_string(),value.to_string()];
        Ok(_key_value)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Wrong parameter"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_arg_key() {
        let result = split_arg("filepath=123").unwrap();
        assert_eq!(result.get(0).unwrap().clone(),"filepath".to_string());
    }

    #[test]
    fn test_split_arg_value() {
        let result = split_arg("filepath=123").unwrap();
        assert_eq!(result.get(1).unwrap().clone(),"123".to_string());
    }
}
