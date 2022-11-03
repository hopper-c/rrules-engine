use serde_json::{ Value, json} ;
use crate::model::condition::Condition;
use crate::model::rule::Rule;

pub mod model;

fn main() {

    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });


}