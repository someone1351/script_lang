use super::int::*;

macro_rules! impl_int_try_into {
    // Match one or more types separated by commas
    ($($ty:ty),+) => {
        // Repeat the implementation for each matched type
        $(
            impl TryInto<$ty> for IntVal {
                type Error = ();

                fn try_into(self) -> Result<$ty, Self::Error> {
                    match self {
                        IntVal::I8(i) => i.try_into().or(Err(())),
                        IntVal::I16(i) => i.try_into().or(Err(())),
                        IntVal::I32(i) => i.try_into().or(Err(())),
                        IntVal::I64(i) => i.try_into().or(Err(())),
                        IntVal::I128(i) => i.try_into().or(Err(())),
                        IntVal::ISize(i) => i.try_into().or(Err(())),
                        IntVal::U8(i) => i.try_into().or(Err(())),
                        IntVal::U16(i) => i.try_into().or(Err(())),
                        IntVal::U32(i) => i.try_into().or(Err(())),
                        IntVal::U64(i) => i.try_into().or(Err(())),
                        IntVal::U128(i) => i.try_into().or(Err(())),
                        IntVal::USize(i) => i.try_into().or(Err(())),
                    }
                }
            }
        )+
    };
}

impl_int_try_into!(i8,i16,i32,i64,i128,isize, u8,u16,u32,u64,u128,usize);

