use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use crate::model::condition::ConditionGroup;

#[derive(Deserialize, Serialize, Debug)]
enum Action {
    Add(i64, i64)
}

#[derive(Debug)]
pub struct Rule { 
    name: &'static str,
    priority: u16,
    conditions: Vec<ConditionGroup>,
    action: Action
}

impl Rule {
    fn execute(&self, fact: Value) -> Value {
        match self.action {
            Action::Add(a, b) => {
                json!(null)
            }
        }
    }
}