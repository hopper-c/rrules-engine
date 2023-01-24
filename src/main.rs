use std::{vec};
use serde_json;
use serde_json::{Value};
use float_cmp::{approx_eq};

// use crate::model::condition::Condition;
// use crate::model::rule::Rule;

//pub mod model;

#[derive(Debug)]
enum LogicalOperator {
    AND,
    OR
}

#[derive(Debug)]
enum Operator {
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
    EqualTo
}

// This only works with integers, but I want to pass in the String straight
fn less_than(variable: &Value, value: &String) -> bool {

    let int_variable: f64 = variable.as_f64().expect("Variable was not a valid number.");
    let int_value: f64 = value.parse().expect("Value was not a valid number.");

    // To remove floating point issues, we'll check if the numbers are equal first
    if approx_eq!(f64, int_variable, int_value, ulps = 2) {
        false
    }
    else {
        int_variable < int_value
    }
}

fn less_than_equal_to(variable: &Value, value: &String) -> bool {

    let int_variable: f64 = variable.as_f64().expect("Variable was not a valid number.");
    let int_value: f64 = value.parse().expect("Value was not a valid number.");

    // To remove floating point issues, we'll check if the numbers are equal first
    if approx_eq!(f64, int_variable, int_value, ulps = 2) {
        true
    }
    else {
        int_variable < int_value
    }
}

fn greater_than(variable: &Value, value: &String) -> bool {

    let int_variable: f64 = variable.as_f64().expect("Variable was not a valid number.");
    let int_value: f64 = value.parse().expect("Value was not a valid number.");

    // To remove floating point issues, we'll check if the numbers are equal first
    if approx_eq!(f64, int_variable, int_value, ulps = 2) {
        false
    }
    else {
        int_variable > int_value
    }
}

fn greater_than_equal_to(variable: &Value, value: &String) -> bool {

    let int_variable: f64 = variable.as_f64().expect("Variable was not a valid number.");
    let int_value: f64 = value.parse().expect("Value was not a valid number.");

    // To remove floating point issues, we'll check if the numbers are equal first
    if approx_eq!(f64, int_variable, int_value, ulps = 2) {
        true
    }
    else{
        int_variable > int_value
    }
}

fn equal_to(variable: &Value, value: &String) -> bool {

    let int_variable: f64 = variable.as_f64().expect("Variable was not a valid number.");
    let int_value: f64 = value.parse().expect("Value was not a valid number.");

    approx_eq!(f64, int_variable, int_value, ulps = 2)
}

fn execute_operator(variable: &Value, operator: &Operator, value: &String) -> bool {
    match(operator) {
        Operator::LessThan => { less_than(variable, value) },
        Operator::LessThanEqualTo => { less_than_equal_to(variable, value) },
        Operator::GreaterThan => { greater_than(variable, value) },
        Operator::GreaterThanEqualTo => { greater_than_equal_to(variable, value) },
        Operator::EqualTo => { equal_to(variable, value) }
    }
}

#[derive(Debug)]
enum ConditionOrGroup {
    Condition { variable: String, operator: Operator, value: String },
    Group { things: Vec<ConditionOrGroup>, logical: LogicalOperator }
}

trait ConditionOperation {
    fn check(&self, json_obj: serde_json::Value) -> bool;
}

fn handle_unwrap(obj: Option<&Value>) -> &Value {
    // Need some better way of error handling
    match obj {
        Some(obj) => obj,
        None => panic!("Cannot unwrap {:?}", obj)
    }
}

fn get_recursively(mut path: Vec<&str>, json: &serde_json::Value) -> serde_json::Value {
    // Get the top path and remove it from the vec
    // Need an expect
    let first_path = path.remove(0);

    let obj = handle_unwrap(json.get(first_path));

    if let Value::Object(other_obj) = obj {
        println!("Attempting to recurse on obj {:?}", obj);
        get_recursively(path, obj)
    }
    else {
        println!("We found the final value {:?}", obj);
        obj.to_owned()
    }
}

fn check_condition(json_obj: serde_json::Value, variable: &String, operator: &Operator, value: &String) -> bool {

    //Parse out the variable, starting simple with just split on '.'
    let split: Vec<&str> = variable.split(".").collect();
    
    //Has to be a faster/better way, but for now, loop
    let obj_value = get_recursively(split, &json_obj);

    println!("{:?}", obj_value);

    let is_fire= execute_operator(&obj_value, operator, value);

    println!("Did our rule fire? {}", is_fire);
    
    true
}

impl ConditionOperation for ConditionOrGroup {
    fn check(&self, json_obj: serde_json::Value) -> bool {
        match(self) {
            ConditionOrGroup::Condition { variable, operator, value } => {
                // Need to get the value of the variable from the object
                // Get the operator function
                // Check the conditions
                check_condition(json_obj, variable, operator, value)
            },
            ConditionOrGroup::Group { things, logical } => {
                // Do the above on the vec
                true
            }
        }
    }
}

fn main() {

    let json = r#"
        { 
            "data": {
                "deeper": 32.5
            }
        }
    "#;

    let json_value = serde_json::from_str(json).expect("Malformed json.");
    
    let condition = ConditionOrGroup::Condition { variable: "data.deeper".to_owned(), operator: Operator::LessThan, value: "35".to_owned() };
    let is_condition = condition.check(json_value);

    let other_condition = ConditionOrGroup::Condition { variable: "object.thing".to_owned(), operator: Operator::EqualTo, value: "literal value".to_owned() };

    let conditions = vec![condition, other_condition];

    let group = ConditionOrGroup::Group { things: conditions, logical: LogicalOperator::AND };

    println!("{:?}", is_condition);
}