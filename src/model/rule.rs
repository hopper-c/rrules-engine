use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use crate::model::condition::ConditionOrGroup;

#[derive(Deserialize, Serialize, Debug)]
pub enum Action {
    Add(i64, i64)
}

#[derive(Debug, Clone)]
pub struct Rule { 
    pub name: &'static str,
    pub priority: u16,
    pub conditions: Vec<ConditionOrGroup>,
    //Temporary
    pub action: &'static str
}

pub trait RuleOperations {
    fn execute(&self, data: String) -> bool;
}

impl RuleOperations for Rule {
    fn execute(&self, data: String) -> bool {
        dbg!(data);
        true
    }
}