use rand::{thread_rng, seq::SliceRandom};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::stdin;

mod testloader {
    pub mod loader;   
}

mod structure {
    pub mod test_structure;
}

mod server {
    pub mod server;
}

fn main() {
    //start server
    //let filename = "D:/dev/rust/testit/src/example/test.json";
    let filename = "D:/dev/rust/testit/src/example/test_ports.json";
    println!("Main function [loaded!]");
    testloader::loader::showinfo();
    let parsed = testloader::loader::read_file(&filename).expect("Error reading file");
    //get unpacked test content as "object"
    let mut test_content = testloader::loader::parse_json(&parsed).unwrap();
    //shuffle test quesitions
    test_content.shuffle(&mut thread_rng());
    println!("Start thread ...");

    //--> temp code 
    let shared_test_content = Arc::new(Mutex::new(test_content));
    //test part to get elements
    let handle = thread::spawn(move || {
        let mut goodAnswers = 0;
        let mut  badAnswers = 0;
        let test_content_ = shared_test_content.lock().unwrap();
        let testlength = test_content_.len();
        println!("Test lenght is: {}",testlength);
        for element in test_content_.iter() {
            //let element: structure::test_structure::TestQAC = test_content.get(0).unwrap().clone();
            let element: structure::test_structure::TestQAC = element.clone();
            println!("{} :",element.get_question());
            let answers: Vec<String> = element.get_answers().clone();
            for (i,answer) in answers.iter().enumerate() { 
                let answer = answer.clone();
                //println!("{}: {:?}",i+1,answer);
            }
            let user_answer = get_user_answer(answers.len()+1).unwrap();
            if user_answer != 0 {
                println!("Your answer: {:?} ", user_answer);
                let corrects: Vec<bool> = element.get_correct().clone();
                let isCorrect = *corrects.get(user_answer-1).unwrap();
                if isCorrect {
                    goodAnswers += 1;
                } else {
                    badAnswers += 1;
                }
            } else {
                println!("Your answer: {:?} , exit from test.", user_answer);
                break;
            }
        }
        println!("Good answers: {}, bad answers: {}",goodAnswers,badAnswers);
    });

    let handle_for_server = thread::spawn( || {
        server::server::start_server();
    });
    
     // Wait for the spawned thread to finish
     handle.join().unwrap();
}

fn get_user_answer(max_input: usize) -> Result<usize, std::io::Error> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    match input.trim().parse::<usize>() {
        Ok(parsed_input) => Ok(parsed_input),
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid input")),
    }
}


