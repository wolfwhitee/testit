use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TestQAC {
    que: String,
    ans: Vec<String>,
    correct: Vec<bool>
}

impl TestQAC {
    pub fn get_question(&self) -> &String {
        &self.que
    }

    pub fn get_answers(&self) -> &Vec<String> {
        &self.ans
    }

    pub fn get_correct(&self) -> &Vec<bool> {
            &self.correct
    }
}