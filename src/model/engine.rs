use crate::model::rule::Rule;

#[derive(Debug)]
pub struct Engine {
    pub rules: Vec<Rule>,
    pub algorithm: &'static str
}

pub trait EngineOperations {
    fn run(&self, data: &str);
}

impl EngineOperations for Engine {

    fn run(&self, data: &str) {
        dbg!("Engine ran.");
    }

}