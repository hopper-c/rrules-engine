use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug)]
enum ConditionOrGroup {
    Condition { predicate: String, operator: String },
    ConditionGroup { conditions: Vec<ConditionOrGroup>, logical: &'static str }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Condition {
    predicate: String,
    operator: String,

}

#[derive(Debug)]
pub struct ConditionGroup {
    group: Vec<Box<ConditionOrGroup>>
}