use crate::parsing::ast;

pub struct Function {

}

impl Function {
    pub fn new(function: &str) -> Self {
        Self {

        }
        
    }

    pub fn eval(&self, x: f32)  -> f32 {
        return x.signum() * f32::sin(x) - x.abs();
    }
}