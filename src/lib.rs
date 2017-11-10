extern crate num_traits;

mod number;
mod value;
mod from;

pub use value::*;
pub use from::*;

pub trait FromValue<T> {
    fn from_value(val: &Value) -> Option<T>;
}

impl FromValue<i64> for i64 {
    fn from_value(val: &Value) -> Option<i64> {
        if let Value::Number(ref n) = *val {
            n.as_i64()
        } else {
            None
        }
    }
}

impl FromValue<u64> for u64 {
    fn from_value(val: &Value) -> Option<u64> {
        if let Value::Number(ref n) = *val {
            n.as_u64()
        } else {
            None
        }
    }
}

impl FromValue<f64> for f64 {
    fn from_value(val: &Value) -> Option<f64> {
        if let Value::Number(ref n) = *val {
            n.as_f64()
        } else {
            None
        }
    }
}

impl FromValue<String> for String {
    fn from_value(val: &Value) -> Option<String> {
        if let Value::String(ref s) = *val {
            Some(s.clone())
        } else {
            None
        }
    }
}

pub fn from_value<T>(val: &Value) -> Option<T>
where
    T: FromValue<T>,
{
    T::from_value(val)

}

#[cfg(test)]
mod test {
    use std::any::Any;
    use std::sync::Arc;
    use super::*;

    #[test]
    fn test_function_cmp() {
        fn f(a: &[Arc<Any>]) -> Result<Arc<Any>, String> {
            Ok(a[0].clone())
        };
        let f1 = Function { f: f };
        let f2 = Function { f: f };
        assert_eq!(f1, f2);
    }
}
