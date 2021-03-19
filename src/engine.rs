use std::collections::HashMap;

pub struct Engine<'a> {
    variables: HashMap<String, &'a dyn Value>, 
}

impl Engine<'_> {

    pub fn run() {

    }

}

pub trait Value {
    fn evaluate(&self) -> i32;
}

pub struct Literal {
    val: i32,
}

impl Value for Literal {
    fn evaluate(&self) -> i32 {
        return self.val;
    }
}

struct AddOperator<'a> {
    lhs: &'a dyn Value, 
    rhs: &'a dyn Value, 
}

impl Value for AddOperator<'_> {
    fn evaluate(&self) -> i32 {
        return self.lhs.evaluate() + self.rhs.evaluate();
    }
}
