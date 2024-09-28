use helpers::argsparser;
use rand::{thread_rng, seq::SliceRandom};
use testhelpers::querandomizer;
use testhelpers::querandomizer_b;
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::stdin;
use std::collections::HashSet;
use std::collections::HashMap;

mod testloader {
    pub mod loader;   
}

mod structure {
    pub mod test_structure;
    pub mod test_structure_adv;
    pub mod args_structure;
}

mod server {
    pub mod server;
}

mod helpers {
    pub mod argsparser;
}

mod testhelpers {
    pub mod querandomizer;
    pub mod querandomizer_b;
}

fn main() {
    println!("Main function [loaded!]");
    println!("Use filepath argument with full path to test file.");
    println!("-filepath=C:\\some_test_dir_path\\test.json");
    println!("if not set, loading example test.");
    //start server
    let run_parameters = argsparser::parse().unwrap();
    println!("Loaded test: {}",run_parameters.get_filepath());
    if run_parameters.get_filepath()!="" {
        let parsed = testloader::loader::read_file(run_parameters.get_filepath()).expect("Error reading file");
        //get unpacked test content as "object"
        let test_version: i8 = testloader::loader::parse_json_probe(&parsed).unwrap();
        println!("Test version: {}",test_version+1);
        if test_version==0 {
            let mut test_content = testloader::loader::parse_json(&parsed).unwrap();
            //shuffle test quesitions
            test_content.shuffle(&mut thread_rng());
            println!("Start thread ...");

            //--> temp code 
            let shared_test_content = Arc::new(Mutex::new(test_content));
            //test part to get elements
            let handle = thread::spawn(move || {
                let mut good_answers = 0;
                let mut  bad_answers = 0;
                let test_content_ = shared_test_content.lock().unwrap();
                let testlength = test_content_.len();
                println!("Test lenght is: {}",testlength);
                for element in test_content_.iter() {
                    let element: structure::test_structure::TestQAC = element.clone();
                    println!("{} :",element.get_question());
                    let answers: Vec<String> = element.get_answers().clone();
                    for (_i,answer) in answers.iter().enumerate() { 
                        let _answer = answer.clone();
                        println!("{}: {:?}",_i+1,answer);
                    }
                    let user_answer = get_user_answer(answers.len()+1).unwrap();
                    if user_answer != 0 {
                        println!("Your answer: {:?} ", user_answer);
                        let corrects: Vec<bool> = element.get_correct().clone();
                        let is_correct = *corrects.get(user_answer-1).unwrap();
                        if is_correct {
                            good_answers += 1;
                        } else {
                            bad_answers += 1;
                        }
                    } else {
                        println!("Your answer: {} , exit from test.", user_answer);
                        break;
                    }
                }
                println!("Good answers: {}, bad answers: {}",good_answers,bad_answers);
            });
       

            let _handle_for_server = thread::spawn( || {
                server::server::start_server();
            });
            
            // Wait for the spawned thread to finish
            handle.join().unwrap();
        } else if test_version==1 {
            println!("Test V2 is not supported yet");
            let test_content = testloader::loader::parse_json_adv(&parsed).unwrap();
            let test_config = test_content.get_config().clone();
            let mut test_content = test_content.get_content().clone();
            test_content.shuffle(&mut thread_rng());
            let mut good_answers = 0;
            let mut  bad_answers = 0;
            let testlength = test_content.len();
            println!("Test lenght is: {}",testlength);
            for element in test_content.iter() {
                let element:structure::test_structure_adv::Content = element.clone();
                let question = element.get_que();
                let answers = element.get_ans().clone();
                let answers_shafled = querandomizer_b::generate_set_of_answers_b(test_config.get_rand_min_answers().clone(), test_config.get_random_max_answers().clone(), test_config.get_rand_set_answers().clone(), answers.clone()).unwrap();
                println!("{} :",question);
                let mut tempMapIndexAns = HashMap::new();
                for (_i,answer) in answers_shafled.get_ans_set().iter().enumerate() { 
                    let _answer = answer.clone();
                    tempMapIndexAns.insert((_i+1).to_string(), answer.to_string());
                    println!("{}: {:?}",_i+1,answer);
                }
                let user_answer = get_user_answer_as_vec(20,tempMapIndexAns).unwrap();
                if user_answer.get(0).unwrap().clone() != "0".to_string() {
                    println!("Your answer: {:?} ", user_answer);
                    println!("Good answer: {:?} ", answers);
                    let is_correct = are_vecs_equal(user_answer,answers);
                    if is_correct {
                        good_answers += 1;
                    } else {
                        bad_answers += 1;
                    }
                    
                } else {
                    println!("Your answer: {:?} , exit from test.", user_answer);
                    println!("Good answers: {}, bad answers: {}",good_answers,bad_answers);
                    break;
                }
                println!("Good answers: {}, bad answers: {}",good_answers,bad_answers);
            }

            
        } else {
            println!("Test version not recognized");
        }
    }
}

fn get_user_answer(_max_input: usize) -> Result<usize, std::io::Error> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    match input.trim().parse::<usize>() {
        Ok(parsed_input) => Ok(parsed_input),
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid input")),
    }
}

fn get_user_answer_as_vec(_max_input: usize, mapping: HashMap<String,String>) -> Result<Vec<String>, std::io::Error> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    if input != "0" {
        let no_spaces: String = input.replace(" ", "");
        let user_answer_indexes: Vec<String> = no_spaces.split(',').map(|s| s.trim().to_string()).collect();
        let mut user_answer: Vec<String> = Vec::new();
        for index in user_answer_indexes {
            if let Some(value) = mapping.get(&index) {
                user_answer.push(value.clone());
            } else {
                // Handle the case where the key is not found
                //return Err(io::Error::new(io::ErrorKind::NotFound, "Key not found in mapping"));
            }
        }
        Ok(user_answer)
    } else {
        Ok(vec!["0".to_string()])
    }

}

fn are_vecs_equal(vec1: Vec<String>, vec2: Vec<String>) -> bool {
    let set1: HashSet<_> = vec1.into_iter().collect();
    let set2: HashSet<_> = vec2.into_iter().collect();
    set1 == set2
}




