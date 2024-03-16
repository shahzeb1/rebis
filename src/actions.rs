use std::collections::HashMap;
use std::fmt;

pub struct Get(pub String);

pub struct Set {
    pub key: String,
    pub value: String,
}

// This value enum is only used in the HashMap
pub enum Value {
    Str(String),
    Int(i32),
}

// DataType of the HashMap
pub type DataType = HashMap<String, Value>;

pub struct Incr(pub String);

pub enum Action {
    GetAction(Get),
    SetAction(Set),
    IncrAction(Incr),
}

pub trait ActionTrait {
    fn execute(&self, data: &mut HashMap<String, Value>);
}

// These should be their own files:

impl ActionTrait for Get {
    fn execute(&self, data: &mut HashMap<String, Value>) {
        if let Some(value) = data.get(&self.0) {
            println!("{}", value);
        } else {
            println!("Key {} not found", self.0);
        }
    }
}

impl ActionTrait for Set {
    fn execute(&self, data: &mut HashMap<String, Value>) {
        let key = self.key.clone();
        let value = self.value.clone();
        if let Ok(int_value) = value.parse::<i32>() {
            data.insert(key, Value::Int(int_value));
        } else {
            data.insert(key, Value::Str(value));
        }
        println!("OK");
    }
}

impl ActionTrait for Incr {
    fn execute(&self, data: &mut HashMap<String, Value>) {
        if let Some(value) = data.get_mut(&self.0) {
            if let Value::Int(int_value) = value {
                *int_value += 1;
                println!("(integer) {}", int_value);
            } else {
                println!("Value is not an integer or out of range");
            }
        } else {
            data.insert(self.0.clone(), Value::Int(1));
            println!("(integer) {}", 1);
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Int(i) => write!(f, "\"{}\"", i),
        }
    }
}
