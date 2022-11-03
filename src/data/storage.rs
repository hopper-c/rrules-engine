use serde_json::{ Value, json };

pub trait Storage {
    fn store_single(rule: Rule) -> bool;
    fn store_multiple(rule: Vec<Rule>) -> bool;

    fn retrieve_single(rule_name: &'static str) -> Rule;
    fn retrieve_all() -> Vec<Rule>;
}

pub impl FileSystem {
    fn new(file_location: &'static str) -> Self {
        FileSystem { file_location: file_location}
    }

    fn location(&self) -> &'static str {
        self.file_location
    }

}

impl Storage for FileSystem {
    fn store_single(rule: Rule) -> bool {
        // Convert to JSON
        

        // Append to file if already exists

    }

    fn store_multiple(rule: Vec<Rule>) -> bool;

    fn retrieve_single(rule_name: &'static str) -> Rule;
    fn retrieve_all() -> Vec<Rule>;
}