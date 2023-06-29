use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use serde_json::Value;

pub trait Arg {}

pub trait Function {
    fn name(&self) -> String;
    fn description(&self) -> String;

    fn run(&self, arg: Value);

    fn _run(&self, arg: Value) {
        self.run(arg);
    }
}

/// All the registered functions
pub struct Functions {
    fns: HashMap<String, Box<dyn Function>>,
    // fn_schema: Value,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            fns: HashMap::new(),
        }
    }

    pub fn insert(&mut self, func: impl Function) {
        // Create schema object for function

        // Register function
        // self.fns.insert(func.name(), );
    }

    /// Call registered function
    pub fn run(&mut self, name: &str, args: Value) {}
}
