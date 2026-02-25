use std::{cmp::Ordering, fmt::Display};

use crate::IntVal;


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
            _ => panic!(""),
        }
    }
    pub fn powi(self,exp:IntVal) -> Result<Self,()> {
        let exp:i32=exp.try_into()?;

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
            _ => panic!(""),
        }
    }
    pub fn sub(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a-b).into(),
            (Self::F64(a), Self::F64(b)) => (a-b).into(),
            _ => panic!(""),
        }
    }
    pub fn mul(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a*b).into(),
            (Self::F64(a), Self::F64(b)) => (a*b).into(),
            _ => panic!(""),
        }
    }
    pub fn div(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a/b).into(),
            (Self::F64(a), Self::F64(b)) => (a/b).into(),
            _ => panic!(""),
        }
    }
    pub fn rem(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => (a%b).into(),
            (Self::F64(a), Self::F64(b)) => (a%b).into(),
            _ => panic!(""),
        }
    }
    pub fn min(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => a.min(b).into(),
            (Self::F64(a), Self::F64(b)) => a.min(b).into(),
            _ => panic!(""),
        }
    }
    pub fn max(self,other:Self) -> Self {
        match (self,other.to_same(self)) {
            (Self::F32(a), Self::F32(b)) => a.max(b).into(),
            (Self::F64(a), Self::F64(b)) => a.max(b).into(),
            _ => panic!(""),
        }
    }

    pub fn clamp(self,min:Self,max:Self) -> Result<Self,()> {
        let min=min.to_same(self);
        let max=max.to_same(self);

        //
        if match min {
            FloatVal::F32(b) => b.is_nan(),
            FloatVal::F64(b) => b.is_nan(),
        } {
            return Err(());
        }

        //
        if match max {
            FloatVal::F32(c) => c.is_nan(),
            FloatVal::F64(c) => c.is_nan(),
        } {
            return Err(());
        }

        //
        if match (min,max) {
            (Self::F32(b), Self::F32(c)) => b>c,
            (Self::F64(b), Self::F64(c)) => b>c,
            _ => panic!(""),
        } {
            return Err(());
        }

        match (self,min,max) {
            (Self::F32(a), Self::F32(b), Self::F32(c)) => Ok(a.clamp(b,c).into()),
            (Self::F64(a), Self::F64(b), Self::F64(c)) => Ok(a.clamp(b,c).into()),
            _ => panic!(""),
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

impl Display for FloatVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatVal::F32(a) => write!(f, "{a}"),
            FloatVal::F64(a) => write!(f, "{a}"),
        }
    }
}

impl TryFrom<IntVal> for FloatVal {
    type Error = ();

    fn try_from(value: IntVal) -> Result<Self, Self::Error> {
        match value {
            IntVal::I8(a) => a.try_into().map(|a|FloatVal::F32(a)).or(Err(())),
            IntVal::I16(a) => a.try_into().map(|a|FloatVal::F32(a)).or(Err(())),
            IntVal::I32(a) => a.try_into().map(|a|FloatVal::F64(a)).or(Err(())),

            IntVal::U8(a) => a.try_into().map(|a|FloatVal::F32(a)).or(Err(())),
            IntVal::U16(a) => a.try_into().map(|a|FloatVal::F32(a)).or(Err(())),
            IntVal::U32(a) => a.try_into().map(|a|FloatVal::F64(a)).or(Err(())),

            // IntVal::I64(a) => ,
            // IntVal::I128(a) => ,
            // IntVal::ISize(a) => ,

            // IntVal::U64(a) => ,
            // IntVal::U128(a) => ,
            // IntVal::USize(a) => ,

            _ => {
                let a:i32=value.try_into()?;
                a.try_into().map(|a|FloatVal::F64(a)).or(Err(()))
            }

        }
    }
}