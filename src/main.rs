use serde_json::{ Value, json} ;

#[derive(Debug)]
pub struct Condition {
    predicate: String,
    operator: String,
    fact: String
}

#[derive(Debug)]
pub struct Rule { 
    priority: u32,
    conditions: Vec<Condition>,
    action: fn(Value) -> bool
}

fn main() {

    let mut conditions: Vec<Condition> = Vec::new();

    let condition = Condition {
        predicate: String::from("Predicate"), 
        operator: String::from("operator"), 
        fact: String::from("fact")
    };

    conditions.push(condition);

    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let first_rule = Rule {
        priority: 1,
        conditions: conditions,
        action: |value: Value| -> bool {
            let mut is_true = false;

            if value["name"] == "John Doe"{
                is_true = true;
            }

            return is_true;
        }
    };

    let is_activated = first_rule.action;
    println!("{}", is_activated(john));
}