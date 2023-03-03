use std::{thread::{self, JoinHandle}, sync::{Arc}};
use crate::model::rule::{Rule, RuleOperations};

#[derive(Debug)]
pub struct Engine {
    pub rules: Vec<Rule>,
    pub algorithm: Algorithm
}

#[derive(Debug)]
pub enum Algorithm {
    Default
}

pub trait EngineOperations {
    fn run(self, data: String) -> bool;
}

impl EngineOperations for Engine {

    fn run(self, data: String) -> bool {

        let clone_data = Arc::new(data);
        let mut handles: Vec<JoinHandle<bool>> = Vec::new();

        // Very basic for loop just to get this to run the rules ingested
        for rule in self.rules {
            let clone_data = Arc::clone(&clone_data);

            let handle = thread::spawn(move || {
                rule.execute(clone_data.to_string())
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("An error occurred when evaluating the rules.");
        }
        dbg!("Engine evaluated all rules.");
        true
    }

}