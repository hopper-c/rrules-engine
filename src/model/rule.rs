use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use crate::model::condition::{ConditionOrGroup, ConditionOperation};

#[derive(Deserialize, Serialize, Debug)]
pub enum Action {
    Add(i64, i64)
}

#[derive(Debug, Clone)]
pub struct Rule { 
    pub name: &'static str,
    pub priority: u16,
    pub conditions: Vec<ConditionOrGroup>,
    // TODO: Transformation Rules will come at a later date
    pub action: &'static str
}

pub trait RuleOperations {
    fn execute(self, data: String) -> bool;
}

impl RuleOperations for Rule {
    fn execute(self, data: String) -> bool {

        let json_value = serde_json::from_str(&data).expect("Malformed json!");

        dbg!("Rules operating on this data: {:?}", &json_value);

        for condition in self.conditions {

            let is_true = condition.check(&json_value);

            if !is_true {
                dbg!("Rule {:?} evaluated to false.", self.name);
                return false;
            }
        }

        dbg!("Rule {:?} evaluated to true.", self.name);
        true
    }
}