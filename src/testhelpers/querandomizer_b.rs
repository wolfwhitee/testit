use std::{io, vec};
use std::collections::HashSet;
use rand::Rng;
use rand::prelude::SliceRandom;

#[derive(Debug)]
pub struct AnsSetB {
    ans: Vec<String>,
    ans_set: Vec<String>
}

impl AnsSetB {
    pub fn new(ans: Vec<String>, ans_set: Vec<String>) -> Self {
        Self { ans, ans_set }
    }
    
    pub fn get_ans(&self) -> &[String] {
        &self.ans
    }
    
    pub fn set_ans(&mut self, ans: Vec<String>) {
        self.ans = ans;
    }
    
    pub fn get_ans_set(&self) -> &[String] {
        &self.ans_set
    }
    
    pub fn set_ans_set(&mut self, ans_set: Vec<String>) {
        self.ans_set = ans_set;
    }

    pub fn shuffle_ans_set(&mut self) {
        self.ans_set.shuffle(&mut rand::thread_rng());
    }
    
}


impl Default for AnsSetB {
    fn default() -> Self {
        AnsSetB {
            ans: Vec::new(),
            ans_set: Vec::new(), 
        }
    }
}



pub fn generate_set_of_answers_b(min: usize,max: usize, i_ans_set: Vec<String>,i_ans: Vec<String>) -> Result<AnsSetB,io::Error> {
    let mut result = AnsSetB{ans: Vec::new(),ans_set: Vec::new()};
    let ava_size = create_unique_vec(i_ans_set.clone(), i_ans.clone()).unwrap().len();
    let _i_ans_set = i_ans_set.clone();
    if i_ans_set.clone().len() < 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input set is to small"))
    }
    let mut _min = min;
    if ava_size < min {
        _min = ava_size;
    }
    let mut _max = max;
    if ava_size < max {
        _max = ava_size;
    }
    println!("Using max: {} and min: {}",_max, _min);
    let _set_len = rand::thread_rng().gen_range(_min..=_max);

    let mut _generated_answers: Vec<String> = Vec::new();
    for (index, value) in i_ans.clone().iter().enumerate() {
        _generated_answers.push(i_ans.get(index).unwrap().clone());
    }

    let _temp_i_ans_set = i_ans_set;
    while _generated_answers.len() <_set_len {
        if let Some(random_string) = _temp_i_ans_set.choose(&mut rand::thread_rng()) {
            println!("{:?}",_temp_i_ans_set);
            if !_generated_answers.contains(random_string) {
                _generated_answers.push(random_string.clone()); 
            }
        } 
    }
    result.set_ans(i_ans);
    result.set_ans_set(_generated_answers);
    result.shuffle_ans_set();
    Ok(result)
}

fn create_unique_vec(vec1: Vec<String>,vec2: Vec<String>) -> Result<Vec<String>,io::Error> {
    let mut _vec1 = vec1;
    let mut _vec2 = vec2;
    _vec1.append(&mut _vec2);
    Ok(_vec1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_set_of_answers_b_for_set_eq_2_1a() {
        let result = generate_set_of_answers_b(2,4,vec!["one".to_string(),"two".to_string()],vec!["four".to_string()]).unwrap();
        println!("{:?}",result);
        assert!(result.get_ans_set().len()==2 || result.get_ans_set().len() == 3,"Expected result lenght of 2 or 3, but got {}", result.get_ans_set().len());
    }

    #[test]
    fn test_generate_set_of_answers_b_for_set_eq_3_1a() {
        let result = generate_set_of_answers_b(3,4,vec!["one".to_string(),"two".to_string(),"three".to_string()],vec!["four".to_string()]).unwrap();
        println!("{:?}",result);
        assert!(result.get_ans_set().len()==3 || result.get_ans_set().len() == 4,"Expected result lenght of 3 or 4, but got {}", result.get_ans_set().len());
    }

    #[test]
    fn test_generate_set_of_answers_b_for_set_eq_3_2a() {
        let result = generate_set_of_answers_b(3,5,vec!["one".to_string(),"two".to_string(),"three".to_string()],vec!["four".to_string(),"five".to_string()]).unwrap();
        println!("{:?}",result);
        assert!(result.get_ans_set().len()==3 || result.get_ans_set().len() == 4 || result.get_ans_set().len() == 5,"Expected result lenght of 3,4 or 5, but got {}", result.get_ans_set().len());
    }

    #[test]
    fn test_generate_set_of_answers_b_for_set_eq_5_2a() {
        let result = generate_set_of_answers_b(4,4,vec!["one".to_string(),"two".to_string(),"three".to_string(),"six".to_string(),"seven".to_string()],vec!["two".to_string(),"five".to_string()]).unwrap();
        println!("{:?}",result);
        assert_eq!(result.get_ans_set().len(),4);
    }
}