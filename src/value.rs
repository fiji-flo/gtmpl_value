use std::any::Any;
use std::fmt;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::sync::Arc;


#[doc(inline)]
pub use number::Number;

/// Function type supported by `gtmpl_value`.
pub type Func = fn(&[Arc<Any>]) -> Result<Arc<Any>, String>;

/// Wrapper struct for `Func`.
#[derive(Clone)]
pub struct Function {
    pub f: Func,
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        self.f as fn(_) -> _ == other.f as fn(_) -> _
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Funtion")
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Funtion")
    }
}

/// Represents a gtmpl value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    String(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Function(Function),
    Number(Number),
}

impl Value {
    pub fn from<T>(t: T) -> Self
    where
        T: Into<Value>,
    {
        t.into()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(ref b) => write!(f, "{}", b),
            Value::String(ref s) => write!(f, "{}", s),
            Value::Function(ref func) => write!(f, "{}", func),
            Value::Number(ref n) => write!(f, "{}", n),
            Value::Array(ref a) => write!(f, "{:?}", a),
            Value::Object(ref o) => write!(f, "{:?}", o),
        }

    }
}
