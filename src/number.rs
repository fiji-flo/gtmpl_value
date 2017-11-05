use std::{u64, i64, f64};
use std::cmp::{Ordering, PartialOrd};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Number {
    n: Num,
}

#[derive(Copy, Clone, Debug)]
enum Num {
    U(u64),
    I(i64),
    F(f64),
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Num) -> Option<Ordering> {
        match (*self, *other) {
            (Num::U(s), Num::U(o)) => s.partial_cmp(&o),
            (Num::I(s), Num::I(o)) => s.partial_cmp(&o),
            (Num::F(s), Num::F(o)) => s.partial_cmp(&o),
            _ => None,
        }
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Num) -> bool {
        match (*self, *other) {
            (Num::U(s), Num::U(o)) => s.eq(&o),
            (Num::I(s), Num::I(o)) => s.eq(&o),
            (Num::F(s), Num::F(o)) => s.eq(&o),
            _ => false,
        }
    }
    fn ne(&self, other: &Num) -> bool {
        match (*self, *other) {
            (Num::U(s), Num::U(o)) => s.ne(&o),
            (Num::I(s), Num::I(o)) => s.ne(&o),
            (Num::F(s), Num::F(o)) => s.ne(&o),
            _ => true,
        }
    }
}

impl Number {
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            Num::U(n) => {
                if n <= (i64::MAX as u64) {
                    Some(n as i64)
                } else {
                    None
                }
            }
            Num::I(n) => Some(n),
            Num::F(n) => {
                if (n as i64) as f64 == n {
                    Some(n as i64)
                } else {
                    None
                }
            }
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            Num::U(n) => Some(n),
            Num::I(n) => if n >= 0 { Some(n as u64) } else { None },
            Num::F(n) => {
                if (n as u64) as f64 == n {
                    Some(n as u64)
                } else {
                    None
                }
            }
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            Num::U(n) => {
                if (n as f64) <= f64::MAX && (n as f64) as u64 == n {
                    Some(n as f64)
                } else {
                    None
                }
            }
            Num::I(n) => {
                if (n as f64) <= f64::MAX && (n as f64) >= f64::MIN && n == (n as f64) as i64 {
                    Some(n as f64)
                } else {
                    None
                }
            }
            Num::F(n) => Some(n),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            Num::U(n) => write!(f, "{}", n),
            Num::I(n) => write!(f, "{}", n),
            Num::F(n) => write!(f, "{}", n),
        }
    }
}


macro_rules! from_i {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Number {
                fn from(n: $ty) -> Self {
                    Number {
                        n: if n < 0 { Num::I(n as i64) } else { Num::U(n as u64) }
                    }
                }
            }
        )*
    };
}

from_i!(
    i64 i32 i16 i8 isize
);


macro_rules! from_u {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Number {
                fn from(n: $ty) -> Self {
                    Number {
                        n: Num::U(n as u64),
                    }
                }
            }
        )*
    };
}

from_u!(
    u64 u32 u16 u8 usize
);

macro_rules! from_f {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Number {
                fn from(n: $ty) -> Self {
                    let num = match n {
                        n if (n as i64) as f64 == (n as f64) => {
                            if n.is_sign_negative() { Num::I(n as i64) } else { Num::U(n as u64) }
                        },
                        n => Num::F(n as f64),
                    };
                    Number {
                        n: num,
                    }
                }
            }
        )*
    };
}

from_f!(
    f64 f32
);

#[cfg(test)]
mod test {
    use super::*;
    use std::u64;

    #[test]
    fn test_i() {
        let num: Number = (-23i8).into();
        assert_eq!(num.as_i64(), Some(-23i64));
        assert_eq!(num.as_u64(), None);
        assert_eq!(num.as_f64(), Some(-23f64));
    }

    #[test]
    fn test_u() {
        let num: Number = 23u8.into();
        assert_eq!(num.as_i64(), Some(23i64));
        assert_eq!(num.as_u64(), Some(23u64));
        assert_eq!(num.as_f64(), Some(23f64));
    }

    #[test]
    fn test_u_max() {
        let num: Number = u64::MAX.into();
        assert_eq!(num.as_i64(), None);
        assert_eq!(num.as_u64(), Some(u64::MAX));
        assert_eq!(num.as_f64(), None);
    }

    #[test]
    fn test_f() {
        let num: Number = 23.42f64.into();
        assert_eq!(num.as_i64(), None);
        assert_eq!(num.as_u64(), None);
        assert_eq!(num.as_f64(), Some(23.42f64));
    }

    #[test]
    fn test_le() {
        let a: Number = 23.0f64.into();
        let b: Number = 24u64.into();
        assert!(a <= b);
    }
}
