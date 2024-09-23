use std::io;
use rand::Rng;
use rand::prelude::SliceRandom;

pub struct AnsSet {
    ans_index: usize,
    ans_set: Vec<String>
}
impl AnsSet {
    pub fn set_ans_set(&mut self, ans_set: Vec<String>) {
        self.ans_set = ans_set;
    }
    
    pub fn get_ans_set(&self) -> &[String] {
        &self.ans_set
    }
    
    pub fn get_ans_index(&self) -> usize {
        self.ans_index
    }
    
    pub fn set_ans_index(&mut self, ans_index: usize) {
        self.ans_index = ans_index;
    }
}

impl Default for AnsSet {
    fn default() -> Self {
        AnsSet {
            ans_index: 99,
            ans_set: Vec::new(), 
        }
    }
}

pub fn generate_set_of_answers(min: usize,max: usize, i_set: Vec<String>, ) -> Result<Vec<String>,io::Error> {
    if i_set.len() < 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input set is to small"))
    }
    let mut _min = min;
    if i_set.len() < min {
        _min = i_set.len();
    }
    let mut _max = max;
    if i_set.len() < max {
        _max = i_set.len();
    }
    println!("Using max: {} and min: {}",_max, _min);
    let _set_len = rand::thread_rng().gen_range(_min..=_max);

    let mut _generated_answers: Vec<String> = Vec::new();
    for num in 0..=_set_len-1 {
        if let Some(random_string) = i_set.clone().choose(&mut rand::thread_rng()) {
            _generated_answers.push(random_string.clone()); 
        } 
    }
    Ok(_generated_answers)
}


pub fn search_or_set_ans(i_set: Vec<String>,_ans: String) -> Result<AnsSet,io::Error> {
    let mut result = AnsSet{ans_index: 99,ans_set: Vec::new()};
    let mut _index:usize  = 99;
    let mut _i_set = i_set.clone();
    for (index, value) in _i_set.iter().enumerate() {
        if value == &_ans {
            _index = index;
            break;
        }
    }
    if _index == 99 {
        let _new_index = rand::thread_rng().gen_range(0..=_i_set.len()-1);
        _index = _new_index;
        _i_set[_index] = _ans;
        
    }
    result.set_ans_index(_index);
    result.set_ans_set(_i_set);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_set_of_answers_for_set_eq_2() {
        let result = generate_set_of_answers(2,4,vec!["one".to_string(),"two".to_string()]).unwrap();
        assert_eq!(result.len(),2);
    }
    #[test]
    fn test_generate_set_of_answers_for_set_eq_3() {
        let result = generate_set_of_answers(2,4,vec!["one".to_string(),"two".to_string(),"three".to_string()]).unwrap();
        assert!(result.len()==2 || result.len() == 3,"Expected result lenght of 2 or 3, but got {}", result.len());
    }

    #[test]
    fn test_search_or_set_ans_for_set_eq_2_entry_exist() {
        let result = generate_set_of_answers(2,2,vec!["one".to_string(),"two".to_string()]).unwrap();
        let answers = search_or_set_ans(result.clone(), "one".to_string()).unwrap();
        println!("{:?}",answers.get_ans_set());
        println!("{}",answers.get_ans_index());
        assert!(answers.get_ans_set().len()==2,"Expected result lenght of 2, but got {}", result.clone().len());
    }

    #[test]
    fn test_search_or_set_ans_for_set_eq_2_no_entry() {
        let result = generate_set_of_answers(2,2,vec!["one".to_string(),"two".to_string()]).unwrap();
        let answers = search_or_set_ans(result.clone(), "three".to_string()).unwrap();
        println!("{:?}",answers.get_ans_set());
        println!("{}",answers.get_ans_index());
        assert!(answers.get_ans_set().len()==2,"Expected result lenght of 2, but got {}", result.clone().len());
    }

    #[test]
    fn test_search_or_set_ans_for_set_eq_2_no_entry_confirm_exist() {
        let result = generate_set_of_answers(2,2,vec!["one".to_string(),"two".to_string()]).unwrap();
        let answers = search_or_set_ans(result.clone(), "three".to_string()).unwrap();
        println!("{:?}",answers.get_ans_set());
        println!("{}",answers.get_ans_index());
        let mut temp_index = 99;
        for (index, value) in answers.get_ans_set().iter().enumerate() {
            if value.clone() == "three".to_string() {
                temp_index = index;
                break;
            }
        }
        assert_eq!(answers.get_ans_index(),temp_index);
    }

    
}