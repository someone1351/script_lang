use std::cmp::Ordering;
use std::fmt::Display;

use crate::impl_int_try_into;
use crate::int_expr_double;
use crate::int_expr_single;
use crate::int_expr_single_else;
use crate::int_expr_single_else_signed;
use crate::int_expr_tripple;
use crate::FloatVal;

// use super::int_intos::*;
// use super::int_froms::*;
// use super::macros::*;

#[derive(Copy,Clone,Debug,Eq, )]
pub enum IntVal {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
}

impl IntVal {
    pub fn to_signed(&self) -> Option<Self> {
        match self.clone() {
            IntVal::U8(i) => i.try_into().ok().map(|j|Self::I8(j)),
            IntVal::U16(i) => i.try_into().ok().map(|j|Self::I16(j)),
            IntVal::U32(i) => i.try_into().ok().map(|j|Self::I32(j)),
            IntVal::U64(i) => i.try_into().ok().map(|j|Self::I64(j)),
            IntVal::U128(i) => i.try_into().ok().map(|j|Self::I128(j)),
            IntVal::USize(i) => i.try_into().ok().map(|j|Self::ISize(j)),
            i => Some(i)
        }
    }
    pub fn to_unsigned(self) -> Option<Self> {
        match self.clone() {
            IntVal::I8(i) => i.is_positive().then(||Self::U8(i as u8)),
            IntVal::I16(i) => i.is_positive().then(||Self::U16(i as u16)),
            IntVal::I32(i) => i.is_positive().then(||Self::U32(i as u32)),
            IntVal::I64(i) => i.is_positive().then(||Self::U64(i as u64)),
            IntVal::I128(i) => i.is_positive().then(||Self::U128(i as u128)),
            IntVal::ISize(i) => i.is_positive().then(||Self::USize(i as usize)),
            i => Some(i)
        }
    }

    pub fn abs(self) -> Self {
        int_expr_single_else_signed!(self,a.abs().into(),self,a)
    }

    pub fn unsigned_abs(self) -> Self {
        int_expr_single_else_signed!(self,a.unsigned_abs().into(),self,a)
    }

    pub fn signum(self) -> Self {
        int_expr_single_else!(self,a.signum().into(),a.min(1).into(),a)
    }
    pub fn is_negative(self) -> bool {
        int_expr_single_else_signed!(self,a.is_negative().into(),false,a)
    }
    pub fn is_positive(self ) -> bool {
        !self.is_negative()
    }
    pub fn to_same(self,other:Self) -> Result<Self,()> {
        match other {
            IntVal::I8(_) => {let x:i8=self.try_into()?;Ok(x.into())},
            IntVal::I16(_) => {let x:i16=self.try_into()?;Ok(x.into())},
            IntVal::I32(_) => {let x:i32=self.try_into()?;Ok(x.into())},
            IntVal::I64(_) => {let x:i64=self.try_into()?;Ok(x.into())},
            IntVal::I128(_) => {let x:i128=self.try_into()?;Ok(x.into())},
            IntVal::ISize(_) => {let x:isize=self.try_into()?;Ok(x.into())},
            IntVal::U8(_) => {let x:u8=self.try_into()?;Ok(x.into())},
            IntVal::U16(_) => {let x:u16=self.try_into()?;Ok(x.into())},
            IntVal::U32(_) => {let x:u32=self.try_into()?;Ok(x.into())},
            IntVal::U64(_) => {let x:u64=self.try_into()?;Ok(x.into())},
            IntVal::U128(_) => {let x:u128=self.try_into()?;Ok(x.into())},
            IntVal::USize(_) => {let x:usize=self.try_into()?;Ok(x.into())},
        }
    }

    pub fn pow(self,exp:Self) -> Result<Self,()> {
        let exp:u32=exp.try_into().or(Err(()))?;
        int_expr_single!(self,a.checked_pow(exp).map(|x|x.into()).ok_or(()),a)
    }
    pub fn neg(self) -> Result<Self,()> {
        int_expr_single_else_signed!(self.to_signed().ok_or(())?,a.checked_neg().map(|x|x.into()).ok_or(()),Err(()),a)
    }
    pub fn add(self,other:Self) -> Result<Self,()> {
        let other=other.to_same(self)?;
        int_expr_double!(self,other,a.checked_add(b).map(|x|x.into()).ok_or(()),a,b)
    }
    pub fn sub(self,other:Self) -> Result<Self,()> {
        let other=other.to_same(self)?;
        int_expr_double!(self,other,a.checked_sub(b).map(|x|x.into()).ok_or(()),a,b)
    }
    pub fn mul(self,other:Self) -> Result<Self,()> {
        let other=other.to_same(self)?;
        int_expr_double!(self,other,a.checked_mul(b).map(|x|x.into()).ok_or(()),a,b)
    }
    pub fn div(self,other:Self) -> Result<Self,()> {
        let other=other.to_same(self)?;
        int_expr_double!(self,other,a.checked_div(b).map(|x|x.into()).ok_or(()),a,b)
    }
    pub fn rem(self,other:Self) -> Result<Self,()> {
        let other=other.to_same(self)?;
        int_expr_double!(self,other,a.checked_rem(b).map(|x|x.into()).ok_or(()),a,b)
    }
    pub fn is_zero(self) -> bool {
        int_expr_single!(self,a==0,a)
    }

    pub fn min(self,other:Self) -> Result<Self,()> {
        let other=other.to_same(self)?;
        int_expr_double!(self,other,Ok(a.min(b).into()),a,b)
    }
    pub fn max(self,other:Self) -> Result<Self,()> {
        let other=other.to_same(self)?;
        int_expr_double!(self,other,Ok(a.max(b).into()),a,b)
    }
    pub fn clamp(self,min:Self,max:Self) -> Result<Self,()> {
        let min=min.to_same(self)?;
        let max=max.to_same(self)?;

        //
        if int_expr_double!(min,max,b>c,b,c) {
            return Err(());
        }

        //
        int_expr_tripple!(self,min,max,Ok(a.clamp(b, c).into()),a,b,c)

    }
}

impl PartialEq for IntVal {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() && other.is_zero() {
            return true;
        }

        if self.is_negative() && other.is_positive() || self.is_positive() && other.is_negative() {
            return false;
        }

        let (a,b)=self.to_same(*other).map(|a|(a,*other)).ok()
            .or_else(||other.to_same(*self).map(|b|(*self,b)).ok())
            .unwrap();

        int_expr_double!(a,b,a==b,a,b)
    }
}

impl PartialOrd for IntVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }

        if self.is_negative() && other.is_positive() {
            return Some(Ordering::Less);
        }

        if self.is_positive() && other.is_negative() {
            return Some(Ordering::Greater);
        }

        let (a,b)=self.to_same(*other).map(|a|(a,*other)).ok()
            .or_else(||other.to_same(*self).map(|b|(*self,b)).ok())
            .unwrap();

        int_expr_double!(a,b,Some(a.cmp(&b)),a,b)
    }
}

impl std::hash::Hash for IntVal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        #[derive(Hash)]
        enum IntHash {
            Pos(u128),
            Neg(u128),
        }
        let h=if self.is_negative() {
            IntHash::Neg(self.unsigned_abs().try_into().unwrap())
        } else {
            IntHash::Pos(self.clone().try_into().unwrap())
        };

        core::mem::discriminant(&h).hash(state);
    }
}

impl Display for IntVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        int_expr_single!(self,write!(f, "{a}"),a)
    }
}

impl_int_try_into!(i8,i16,i32,i64,i128,isize, u8,u16,u32,u64,u128,usize);


impl From<i8> for IntVal {
    fn from(v: i8) -> Self {
        Self::I8(v)
    }
}

impl From<i16> for IntVal {
    fn from(v: i16) -> Self {
        Self::I16(v)
    }
}


impl From<i32> for IntVal {
    fn from(v: i32) -> Self {
        Self::I32(v)
    }
}

impl From<i64> for IntVal {
    fn from(v: i64) -> Self {
        Self::I64(v)
    }
}

impl From<i128> for IntVal {
    fn from(v: i128) -> Self {
        Self::I128(v)
    }
}

impl From<isize> for IntVal {
    fn from(v: isize) -> Self {
        Self::ISize(v)
    }
}

impl From<u8> for IntVal {
    fn from(v: u8) -> Self {
        Self::U8(v)
    }
}

impl From<u16> for IntVal {
    fn from(v: u16) -> Self {
        Self::U16(v)
    }
}


impl From<u32> for IntVal {
    fn from(v: u32) -> Self {
        Self::U32(v)
    }
}

impl From<u64> for IntVal {
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}

impl From<u128> for IntVal {
    fn from(v: u128) -> Self {
        Self::U128(v)
    }
}

impl From<usize> for IntVal {
    fn from(v: usize) -> Self {
        Self::USize(v)
    }
}

impl From<FloatVal> for IntVal {
    fn from(value: FloatVal) -> Self {
        match value {
            FloatVal::F32(a) => (a as i32).into(),
            FloatVal::F64(a) => (a as i64).into(),
        }
    }
}