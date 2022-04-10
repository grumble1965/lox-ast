use std::cmp::*;
use std::fmt;
use std::ops::*;

#[derive(Debug, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    ArithmeticError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(n) => write!(f, "{n}"),
            Object::Str(s) => write!(f, "\"{s}\""),
            Object::Bool(b) => write!(f, "{b}"),
            Object::Nil => write!(f, "Nil"),
            Object::ArithmeticError => panic!("Can't print ArithmeticErrors"),
        }
    }
}

//
// Unary Operations
//
impl Neg for Object {
    type Output = Object;

    fn neg(self) -> Object {
        match self {
            Object::Num(n) => Object::Num(-n),
            _ => Object::ArithmeticError,
        }
    }
}

impl Not for Object {
    type Output = Object;

    fn not(self) -> Object {
        match self {
            Object::Nil | Object::Bool(false) => Object::Bool(true),
            Object::Num(_) | Object::Str(_) | Object::Bool(true) => Object::Bool(false),
            _ => Object::ArithmeticError,
        }
    }
}

//
// Comparisons
//
impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Object::Num(n1), Object::Num(n2)) => n1.partial_cmp(n2),
            (Object::Str(s1), Object::Str(s2)) => s1.partial_cmp(s2),
            (Object::Bool(b1), Object::Bool(b2)) => b1.partial_cmp(b2),
            _ => None,
        }
    }
}

//
// Equality
//
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Num(n1), Object::Num(n2)) => n1 == n2,
            (Object::Str(s1), Object::Str(s2)) => s1 == s2,
            (Object::Bool(b1), Object::Bool(b2)) => b1 == b2,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }
}
