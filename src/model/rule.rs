use serde::{ Deserialize, Serialize };
use serde_json::{ Value, json };
use crate::model::condition::ConditionOrGroup;

#[derive(Deserialize, Serialize, Debug)]
pub enum Action {
    Add(i64, i64)
}

#[derive(Debug)]
pub struct Rule { 
    pub name: &'static str,
    pub priority: u16,
    pub conditions: Vec<ConditionOrGroup>,
    //Temporary
    pub action: &'static str
}

// impl Rule {
//     fn execute(&self, fact: Value) -> Value {
//         match self.action {
//             Action::Add(a, b) => {
//                 json!(null)
//             }
//         }
//     }
// }