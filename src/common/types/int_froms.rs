use super::int::*;

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