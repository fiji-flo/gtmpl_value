use std::borrow::Cow;
use std::collections::HashMap;

use value::{Func, Function, Value};

macro_rules! from_num {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Number(n.into())
                }
            }
        )*
    };
}

from_num! {
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
    f32 f64
}

impl From<bool> for Value {
    /// Convert boolean to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::Value;
    ///
    /// let b = false;
    /// let x: Value = b.into();
    /// ```
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl<'a> From<&'a String> for Value {
    /// Convert &String to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::Value;
    ///
    /// let s: &String = &"foobar".to_owned();
    /// let x: Value = s.into();
    /// ```
    fn from(s: &'a String) -> Self {
        Value::String(s.clone())
    }
}

impl From<String> for Value {
    /// Convert String to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::Value;
    ///
    /// let s: String = "foobar".to_owned();
    /// let x: Value = s.into();
    /// ```
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl<'a> From<&'a str> for Value {
    /// Convert &str to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::Value;
    ///
    /// let s = "foobar";
    /// let x: Value = s.into();
    /// ```
    fn from(f: &str) -> Self {
        Value::String(f.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    /// Convert Cow<str> to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Borrowed("foobar");
    /// let x: Value = s.into();
    /// ```
    fn from(f: Cow<'a, str>) -> Self {
        Value::String(f.to_string())
    }
}

impl From<Func> for Value {
    /// Convert Func to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::{Func, Value};
    /// use std::any::Any;
    /// use std::sync::Arc;
    ///
    /// fn f(a: &[Arc<Any>]) -> Result<Arc<Any>, String> {
    ///     Ok(a[0].clone())
    /// };
    /// let x: Value = (f as Func).into();
    /// ```
    fn from(f: Func) -> Self {
        Value::Function(Function { f: f })
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value> + Clone,
{
    /// Convert Vec to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::Value;
    ///
    /// let v = vec!(1, 2, 3);
    /// let x: Value = v.into();
    /// ```
    fn from(f: Vec<T>) -> Self {
        Value::Array(f.iter().cloned().map(|x| x.into()).collect())
    }
}

impl<'a, T> From<&'a [T]> for Value
where
    T: Into<Value> + Clone,
{
    /// Convert Slice to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::Value;
    ///
    /// let v: &[i32] = &[1, 2, 3];
    /// let x: Value = v.into();
    /// ```
    fn from(f: &'a [T]) -> Self {
        Value::Array(f.iter().cloned().map(|x| x.into()).collect())
    }
}

impl<T> From<HashMap<String, T>> for Value
where
    T: Into<Value> + Clone,
{
    /// Convert HashMap<String, T> to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gtmpl_value::Value;
    /// use std::collections::HashMap;
    ///
    /// let mut m = HashMap::new();
    /// m.insert("hello".to_owned(), 123);
    /// let x: Value = m.into();
    /// ```
    fn from(f: HashMap<String, T>) -> Self {
        Value::Object(
            f.iter()
                .map(|(s, x)| (s.clone(), x.clone().into()))
                .collect(),
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec() {
        let val: Value = vec![1, 2, 3].into();
        if let Value::Array(array) = val {
            assert_eq!(array[0], 1.into());
            assert_eq!(array[1], 2.into());
            assert_eq!(array[2], 3.into());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_slice() {
        let slice: &[u8] = &[1, 2, 3];
        let val: Value = slice.into();
        if let Value::Array(array) = val {
            assert_eq!(array[0], 1.into());
            assert_eq!(array[1], 2.into());
            assert_eq!(array[2], 3.into());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_map() {
        let mut m = HashMap::new();
        m.insert("a".to_owned(), 1);
        m.insert("b".to_owned(), 2);
        let val: Value = m.into();
        if let Value::Object(obj) = val {
            assert_eq!(obj.get("a"), Some(&(1.into())));
            assert_eq!(obj.get("b"), Some(&(2.into())));
        } else {
            assert!(false);
        }
    }
}
