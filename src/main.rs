enum Operator {
    lessThan,
    lessThanEqualTo,
    greaterThan,
    greaterThanEqualTo,
    equal,
    
}

struct Condition {
    predicate: String,
    operator: Operator,
    fact: String
}

#[derive(Debug)]
struct Rule { 
    priority: u32,
    condition: Vec<String>,
    operator: String,
    fact: String,
    action: || -> ()
}

fn main() {
    let first_rule = Rule {
        priority: 1,
        condition: Vec<String> { "Stuff" },
        operator: "=",
        fact: "Stuff"
    };

}
