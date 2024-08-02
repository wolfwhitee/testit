use helpers::argsparser;
use rand::{thread_rng, seq::SliceRandom};
use structure::args_structure;
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::stdin;

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

fn main() {
    println!("Main function [loaded!]");
    //start server
    let run_parameters = argsparser::parse().unwrap();
    if run_parameters.get_filepath()!="" {
        let parsed = testloader::loader::read_file(run_parameters.get_filepath()).expect("Error reading file");
        //get unpacked test content as "object"
        let test_version: i8 = testloader::loader::parse_json_probe(&parsed).unwrap();
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
                        println!("Your answer: {:?} , exit from test.", user_answer);
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
            let mut test_content = testloader::loader::parse_json_adv(&parsed).unwrap();
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


