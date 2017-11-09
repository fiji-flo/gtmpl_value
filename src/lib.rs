extern crate num_traits;

mod number;
mod value;
mod from;

pub use value::*;
pub use from::*;

/// Help to write new functions for gtmpl.
#[macro_export]
macro_rules! gtmpl_fn {
    (
        $(#[$outer:meta])*
        fn $name:ident() -> Result<$otyp:ty, String>
        { $($body:tt)* }
    ) => {
        $(#[$outer])*
        pub fn $name(args: &[Arc<Any>]) -> Result<Arc<Any>, String> {
            fn inner() -> Result<$otyp, String> {
                $($body)*
            }
            Ok(Arc::new(Value::from(inner()?)))
        }
    };
    (
        $(#[$outer:meta])*
        fn $name:ident($arg0:ident : $typ0:ty$(, $arg:ident : $typ:ty),*) -> Result<$otyp:ty, String>
        { $($body:tt)* }
    ) => {
        $(#[$outer])*
        pub fn $name(args: &[::std::sync::Arc<::std::any::Any>]) -> Result<::std::sync::Arc<::std::any::Any>, String> {
            #[allow(unused_mut)]
            let mut args = args;
            if args.is_empty() {
                return Err(String::from("at least one argument required"));
            }
            let x = &args[0];
            let $arg0 = x.downcast_ref::<::gtmpl_value::Value>()
                .ok_or_else(|| "unable to downcast".to_owned())?;
            let $arg0: $typ0 = ::gtmpl_value::from_value($arg0)
                .ok_or_else(|| "unable to convert from Value".to_owned())?;
            $(args = &args[1..];
              let x = &args[0];
              let $arg = x.downcast_ref::<::gtmpl_value::Value>()
              .ok_or_else(|| "unable to downcast".to_owned())?;
              let $arg: $typ = ::gtmpl_value::from_value($arg)
                .ok_or_else(|| "unable to convert from Value".to_owned())?;)*;
            fn inner($arg0 : $typ0, $($arg : $typ,)*) -> Result<$otyp, String> {
                $($body)*
            }
            let ret: ::gtmpl_value::Value = inner($arg0, $($arg),*)?.into();
            Ok(::std::sync::Arc::new(ret))
        }
    }
}





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
