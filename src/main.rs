use std::borrow::Borrow;
use std::{vec};
use model::condition::{ ConditionOrGroup, Operator, self };
use serde_json;

pub mod model;
use model::engine::{Engine, EngineOperations, Algorithm};
use model::rule::Rule;
// use crate::model::condition::{ ConditionOrGroup, Operator, LogicalOperator };
// use crate::model::rule::Rule;



fn main() {

    let json = r#"
        { 
            "data": {
                "deeper": 32.5,
                "otherDeeper": 55,
                "group1": {
                    "condition1": 30,
                    "condition2": 35
                },
                "group2": {
                    "condition3": 30,
                    "condition4": 32
                }
            }
        }
    "#.to_string();
    let condition_true = ConditionOrGroup::Condition { variable: "data.deeper".to_owned(), operator: Operator::LessThan, value: "35".to_owned() };
    let condition_true2 = ConditionOrGroup::Condition { variable: "data.deeper".to_owned(), operator: Operator::LessThan, value: "36".to_owned() };
    let condition_true3 = ConditionOrGroup::Condition { variable: "data.deeper".to_owned(), operator: Operator::LessThan, value: "36".to_owned() };
    let condition_true4 = ConditionOrGroup::Condition { variable: "data.deeper".to_owned(), operator: Operator::LessThan, value: "36".to_owned() };
    let condition_false = ConditionOrGroup::Condition { variable: "data.deeper".to_owned(), operator: Operator::LessThan, value: "5".to_owned() };
    let condition_false2 = ConditionOrGroup::Condition { variable: "data.deeper".to_owned(), operator: Operator::LessThan, value: "5".to_owned() };

    let true_conditions = vec![condition_true, condition_true2];
    let true_condition_group = ConditionOrGroup::Group { conditions: true_conditions, logical: condition::LogicalOperator::AND };

    let semi_true_conditions = vec![condition_true3, condition_false];
    let other_semi_true_conditions = vec![condition_true4, condition_false2];
    let other_true_condition_group = ConditionOrGroup::Group { conditions: semi_true_conditions, logical: condition::LogicalOperator::OR };
    let false_condition_group = ConditionOrGroup::Group { conditions: other_semi_true_conditions, logical: condition::LogicalOperator::AND };

    let true_condition1_vec = vec![true_condition_group];
    let true_condition2_vec = vec![other_true_condition_group];
    let false_condition_vec = vec![false_condition_group];


    let rule = Rule {
        name: "Testing1",
        priority: 1,
        conditions: true_condition1_vec,
        action: "Fake"
    };

    let rule2 = Rule {
        name: "Testing2",
        priority: 1,
        conditions: true_condition2_vec,
        action: "Fake"
    };

    let rule3 = Rule {
        name: "Testing3",
        priority: 1,
        conditions: false_condition_vec,
        action: "Fake"
    };

    let rules = vec![rule, rule2, rule3];

    let engine = Engine {
        rules: rules,
        algorithm: Algorithm::Default
    };

    engine.run(json);




    // let json_value = serde_json::from_str(json).expect("Malformed json.");
    
    // let condition = ConditionOrGroup::Condition { variable: "data.deeper".to_owned(), operator: Operator::LessThan, value: "30".to_owned() };
    // let is_condition_true = condition.check(&json_value);

    // let other_condition = ConditionOrGroup::Condition { variable: "data.otherDeeper".to_owned(), operator: Operator::EqualTo, value: "55".to_owned() };

    // let conditions = vec![condition, other_condition];

    // let group = ConditionOrGroup::Group { conditions: conditions, logical: LogicalOperator::OR };
    // let is_group_true = group.check(&json_value);

    // let condition1 = ConditionOrGroup::Condition { variable: "data.group1.condition1".to_owned(), operator: Operator::EqualTo, value: "30".to_owned() };
    // let condition2 = ConditionOrGroup::Condition { variable: "data.group1.condition2".to_owned(), operator: Operator::GreaterThan, value: "30".to_owned() };
    // let condition3 = ConditionOrGroup::Condition { variable: "data.group2.condition3".to_owned(), operator: Operator::LessThanEqualTo, value: "30".to_owned() };
    // let condition4 = ConditionOrGroup::Condition { variable: "data.group2.condition4".to_owned(), operator: Operator::LessThan, value: "30".to_owned() };

    // let group1_conditions = vec![condition1, condition2];
    // let group2_conditions = vec![condition3, condition4];

    // let group1 = ConditionOrGroup::Group { conditions: group1_conditions, logical: LogicalOperator::OR };
    // let group2 = ConditionOrGroup::Group { conditions: group2_conditions, logical: LogicalOperator::AND };

    // let groups = vec![group1, group2];

    // let group_of_groups = ConditionOrGroup::Group { conditions: groups, logical: LogicalOperator::AND };

    // let are_groups_true = group_of_groups.check(&json_value);

    // dbg!("{:?}", is_condition_true);
    // dbg!("{:?}", is_group_true);
    // dbg!("{:?}", are_groups_true);
}