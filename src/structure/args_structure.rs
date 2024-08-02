use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ArgsS {
    filepath: String
}

impl ArgsS {

    pub fn new(filepath: String) -> Self {
        ArgsS { filepath }
    }

    pub fn get_filepath(&self) -> &String {
        &self.filepath
    }
    
    pub fn set_filepath(&mut self, filepath: String) {
        self.filepath = filepath;
    }
}