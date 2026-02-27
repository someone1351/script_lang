use std::{cmp::Ordering, fmt::Display, num::ParseFloatError, str::FromStr};

use crate::{IntVal, IntValErr};


#[derive(Copy,Clone,Debug,)]
pub enum FloatVal {
    F32(f32),
    F64(f64),
}

impl FloatVal {
    pub fn to_same(self,other:Self) -> Self {
        match other {
            Self::F32(_) => self.into(),
            Self::F64(_) => self.into(),
        }
    }

    pub fn abs(self) -> Self {
        match self {
            Self::F32(a) => a.abs().into(),
            Self::F64(a) => a.abs().into(),
        }
    }
    pub fn signum(self) -> Self {
        match self {
            Self::F32(a) => a.signum().into(),
            Self::F64(a) => a.signum().into(),
        }
    }
    pub fn is_negative(self) -> bool {
        match self {
            Self::F32(a) => a.is_sign_negative(),
            Self::F64(a) => a.is_sign_negative(),
        }
    }
    pub fn is_positive(self ) -> bool {
        match self {
            Self::F32(a) => a.is_sign_positive(),
            Self::F64(a) => a.is_sign_positive(),
        }
    }

    pub fn powf(self,exp:Self) -> Self {
        match (self,exp.to_same(self)) {
            (Self::F32(a), Self::F32(exp)) => a.powf(exp).into(),
            (Self::F64(a), Self::F64(exp)) => a.powf(exp).into(),
            _ => panic!("never reached"),
        }
    }
    pub fn checked_powi(self,exp:IntVal) -> Result<Self,IntValErr> {
        let exp:i32=exp.try_into()?;//.or(Err(FloatValErr))

        match self {
            Self::F32(a) => Ok(a.powi(exp).into()),
            Self::F64(a) => Ok(a.powi(exp).into()),
        }
    }
    pub fn neg(self) -> Self {
        match self {
            Self::F32(a) => (-a).into(),
            Self::F64(a) => (-a).into(),
        }
    }
    pub fn add(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a+b).into(),
            (Self::F64(a), Self::F64(b)) => (a+b).into(),
            _ => panic!("never reached"),
        }
    }
    pub fn sub(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a-b).into(),
            (Self::F64(a), Self::F64(b)) => (a-b).into(),
            _ => panic!("never reached"),
        }
    }
    pub fn mul(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a*b).into(),
            (Self::F64(a), Self::F64(b)) => (a*b).into(),
            _ => panic!("never reached"),
        }
    }
    pub fn div(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a/b).into(),
            (Self::F64(a), Self::F64(b)) => (a/b).into(),
            _ => panic!("never reached"),
        }
    }
    pub fn rem(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a%b).into(),
            (Self::F64(a), Self::F64(b)) => (a%b).into(),
            _ => panic!("never reached"),
        }
    }
    pub fn min(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => a.min(b).into(),
            (Self::F64(a), Self::F64(b)) => a.min(b).into(),
            _ => panic!("never reached"),
        }
    }
    pub fn max(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => a.max(b).into(),
            (Self::F64(a), Self::F64(b)) => a.max(b).into(),
            _ => panic!("never reached"),
        }
    }

    pub fn checked_clamp(self,min:Self,max:Self) -> Result<Self,FloatValErr> {
        let min=min.to_same(self);
        let max=max.to_same(self);

        //
        if match min {
            FloatVal::F32(b) => b.is_nan(),
            FloatVal::F64(b) => b.is_nan(),
        } {
            return Err(FloatValErr);
        }

        //
        if match max {
            FloatVal::F32(c) => c.is_nan(),
            FloatVal::F64(c) => c.is_nan(),
        } {
            return Err(FloatValErr);
        }

        //
        if match (min,max) {
            (Self::F32(b), Self::F32(c)) => b>c,
            (Self::F64(b), Self::F64(c)) => b>c,
            _ => panic!("never reached"),
        } {
            return Err(FloatValErr);
        }

        match (self,min,max) {
            (Self::F32(a), Self::F32(b), Self::F32(c)) => Ok(a.clamp(b,c).into()),
            (Self::F64(a), Self::F64(b), Self::F64(c)) => Ok(a.clamp(b,c).into()),
            _ => panic!("never reached"),
        }
    }

    pub fn is_zero(self) -> bool {
        match self {
            FloatVal::F32(a) => a==0.0,
            FloatVal::F64(a) => a==0.0,
        }
    }
    pub fn floor(self) -> Self {
        match self {
            FloatVal::F32(a) => a.floor().into(),
            FloatVal::F64(a) => a.floor().into(),
        }
    }
    pub fn ceil(self) -> Self {
        match self {
            FloatVal::F32(a) => a.ceil().into(),
            FloatVal::F64(a) => a.ceil().into(),
        }
    }
    pub fn round(self) -> Self {
        match self {
            FloatVal::F32(a) => a.round().into(),
            FloatVal::F64(a) => a.round().into(),
        }
    }
    pub fn trunc(self) -> Self {
        match self {
            FloatVal::F32(a) => a.trunc().into(),
            FloatVal::F64(a) => a.trunc().into(),
        }
    }
    pub fn fract(self) -> Self {
        match self {
            FloatVal::F32(a) => a.fract().into(),
            FloatVal::F64(a) => a.fract().into(),
        }
    }
    pub fn is_infinite(self) -> bool {
        match self {
            FloatVal::F32(a) => a.is_infinite(),
            FloatVal::F64(a) => a.is_infinite(),
        }
    }
    pub fn is_nan(self) -> bool {
        match self {
            FloatVal::F32(a) => a.is_nan(),
            FloatVal::F64(a) => a.is_nan(),
        }
    }
    pub fn sqrt(self) -> Self {
        match self {
            FloatVal::F32(a) => a.sqrt().into(),
            FloatVal::F64(a) => a.sqrt().into(),
        }
    }
    pub fn cbrt(self) -> Self {
        match self {
            FloatVal::F32(a) => a.cbrt().into(),
            FloatVal::F64(a) => a.cbrt().into(),
        }
    }
    pub fn cos(self) -> Self {
        match self {
            FloatVal::F32(a) => a.cos().into(),
            FloatVal::F64(a) => a.cos().into(),
        }
    }
    pub fn acos(self) -> Self {
        match self {
            FloatVal::F32(a) => a.acos().into(),
            FloatVal::F64(a) => a.acos().into(),
        }
    }
    pub fn sin(self) -> Self {
        match self {
            FloatVal::F32(a) => a.sin().into(),
            FloatVal::F64(a) => a.sin().into(),
        }
    }
    pub fn asin(self) -> Self {
        match self {
            FloatVal::F32(a) => a.asin().into(),
            FloatVal::F64(a) => a.asin().into(),
        }
    }
    pub fn tan(self) -> Self {
        match self {
            FloatVal::F32(a) => a.tan().into(),
            FloatVal::F64(a) => a.tan().into(),
        }
    }
    pub fn atan(self) -> Self {
        match self {
            FloatVal::F32(a) => a.atan().into(),
            FloatVal::F64(a) => a.atan().into(),
        }
    }
    pub fn atan2(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (FloatVal::F32(a),FloatVal::F32(b)) => a.atan2(b).into(),
            (FloatVal::F64(a),FloatVal::F64(b)) => a.atan2(b).into(),
            _ => panic!("never reached"),
        }
    }
}

impl PartialEq for FloatVal {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::F32(a), Self::F32(b)) => a == b,
            (Self::F64(a), Self::F64(b)) => a == b,
            (Self::F32(a), Self::F64(b)) => (a as f64)==b,
            (Self::F64(a), Self::F32(b)) => a == (b as f64),
        }
    }
}

impl PartialOrd for FloatVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Self::F32(a), Self::F32(b)) => a.partial_cmp(&b),
            (Self::F64(a), Self::F64(b)) => a.partial_cmp(&b),
            (Self::F32(a), Self::F64(b)) => (a as f64).partial_cmp(&b),
            (Self::F64(a), Self::F32(b)) => a.partial_cmp(&(b as f64)),
        }
    }
}

impl From<f32> for FloatVal {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}
impl From<f64> for FloatVal {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl FromStr for FloatVal {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>().map(|x|x.into())
    }
}

// impl ToString for FloatVal {
//     fn to_string(&self) -> String {
//         format!("{self}")
//     }
// }

impl TryFrom<IntVal> for FloatVal {
    type Error = FloatValErr;

    fn try_from(value: IntVal) -> Result<Self, Self::Error> {
        match value {
            IntVal::I8(a) => a.try_into().map(|a|FloatVal::F32(a)).or(Err(FloatValErr)),
            IntVal::I16(a) => a.try_into().map(|a|FloatVal::F32(a)).or(Err(FloatValErr)),
            IntVal::I32(a) => a.try_into().map(|a|FloatVal::F64(a)).or(Err(FloatValErr)),

            IntVal::U8(a) => a.try_into().map(|a|FloatVal::F32(a)).or(Err(FloatValErr)),
            IntVal::U16(a) => a.try_into().map(|a|FloatVal::F32(a)).or(Err(FloatValErr)),
            IntVal::U32(a) => a.try_into().map(|a|FloatVal::F64(a)).or(Err(FloatValErr)),

            // IntVal::I64(a) => ,
            // IntVal::I128(a) => ,
            // IntVal::ISize(a) => ,

            // IntVal::U64(a) => ,
            // IntVal::U128(a) => ,
            // IntVal::USize(a) => ,

            _ => {
                let a:i32=value.try_into().or(Err(FloatValErr))?;
                a.try_into().map(|a|FloatVal::F64(a)).or(Err(FloatValErr))
            }

        }
    }
}


impl Display for FloatVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatVal::F32(a) => write!(f, "{a}"),
            FloatVal::F64(a) => write!(f, "{a}"),
        }
    }
}

impl Default for FloatVal {
    fn default() -> Self {
        Self::F64(0.0)
    }
}

#[derive(Debug)]
pub struct FloatValErr;

impl Display for FloatValErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FloatValErr")
    }
}