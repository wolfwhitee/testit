use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TestSet {
    metadata: Metadata,
    config: Config,
    content: Vec<Content>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Metadata {
    version: String,
    author: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
        rand_questions: bool,
        rand_answers: bool,
        rand_max_answers : usize,
        rand_min_answers : usize,
        rand_set_answers : Vec<String>,
}

impl Config {
    pub fn is_random_questions(&self) -> bool {
        self.rand_questions
    }
    pub fn is_random_answers(&self) -> bool {
        self.rand_answers
    }
    pub fn get_random_max_answers(&self) -> usize {
        self.rand_min_answers
    }
    pub fn get_rand_min_answers(&self) -> usize {
        self.rand_max_answers
    }
    pub fn get_rand_set_answers(&self) -> Vec<String> {
        self.rand_set_answers.clone()
    }

}

#[derive(Debug, Deserialize, Clone)]
pub struct Content {
    que: String,
    ans: Vec<String>,
}

impl Content {
    pub fn get_que(&self) -> &String {
        &self.que
    }

    pub fn get_ans(&self) -> &Vec<String> {
        &self.ans
    }
}

impl TestSet {
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_content(&self) -> &Vec<Content> {
        &self.content
    }
}