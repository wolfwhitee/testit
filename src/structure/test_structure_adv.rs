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
        rand_max_answers : u8,
        rand_min_answers : u8,
        rand_set_answers : Vec<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Content {
    que: String,
    ans: Vec<String>,
}

impl TestSet {
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_content(&self) -> &Vec<Content> {
        &self.content
    }
}