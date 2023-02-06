use float_cmp::{ approx_eq };
use serde_json::{ Value };

#[derive(Debug)]
pub enum LogicalOperator {
    AND,
    OR
}

#[derive(Debug)]
pub enum Operator {
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
    EqualTo
}

#[derive(Debug)]
pub enum ConditionOrGroup {
    Condition { variable: String, operator: Operator, value: String },
    Group { conditions: Vec<ConditionOrGroup>, logical: LogicalOperator }
}

// This only works with numbers, but I want to pass in the String straight
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

pub trait ConditionOperation {
    fn check(&self, json_obj: &serde_json::Value) -> bool;
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
        dbg!("Attempting to recurse on obj {:?}", obj);
        get_recursively(path, obj)
    }
    else {
        dbg!("We found the final value {:?}", obj);
        obj.to_owned()
    }
}

fn check_condition(json_obj: &serde_json::Value, variable: &String, operator: &Operator, value: &String) -> bool {

    //Parse out the variable, starting simple with just split on '.'
    //TODO: Implement array searching and more complex queries
    let split: Vec<&str> = variable.split(".").collect();
    
    let obj_value = get_recursively(split, &json_obj);

    dbg!("{:?}", &obj_value);

    let is_fire= execute_operator(&obj_value, operator, value);

    dbg!("Did our rule fire? {}", is_fire);
    
    is_fire
}

impl ConditionOperation for ConditionOrGroup {
    fn check(&self, json_obj: &serde_json::Value) -> bool {
        match(self) {
            ConditionOrGroup::Condition { variable, operator, value } => {
                check_condition(json_obj, variable, operator, value)
            },
            ConditionOrGroup::Group { conditions, logical } => {
                let did_fire = match logical {
                    LogicalOperator::AND => {
                        let mut is_true = true;

                        // Probably better to do this in a thread. Will update later
                        for condition in conditions {
                            is_true = match condition {
                                ConditionOrGroup::Group { conditions, logical } => { 
                                    dbg!("Found a group: {:?}", condition);
                                    condition.check(&json_obj) 
                                },
                                ConditionOrGroup::Condition { variable, operator, value } => { check_condition(json_obj, variable, operator, value) }
                            };

                            // If we ever find a false on the group of conditions above, immediately break
                            if !is_true { break; }
                        };

                        is_true
                    },
                    LogicalOperator::OR => {
                        let mut is_true = false;

                        for condition in conditions {
                            is_true = match condition {
                                ConditionOrGroup::Group { conditions, logical } => { 
                                    dbg!("Found a group: {:?}", condition);
                                    condition.check(&json_obj) 
                                },
                                ConditionOrGroup::Condition { variable, operator, value } => { check_condition(json_obj, variable, operator, value) }
                            };

                            // If we ever find a true on the group of conditions above, immediately break
                            if is_true { break; }
                        };

                        is_true
                    }
                };

                did_fire
            }
        }
    }
}