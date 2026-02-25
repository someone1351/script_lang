//$lhs.to_same($rhs).ok_or(())?

#[macro_export]
macro_rules! int_expr_double {
    ($lhs:expr,$rhs:expr,$expr:expr,$a:ident,$b:ident) => {
        match ($lhs,$rhs) {
            (IntVal::I8($a), IntVal::I8($b)) => $expr,
            (IntVal::I16($a), IntVal::I16($b)) => $expr,
            (IntVal::I32($a), IntVal::I32($b)) => $expr,
            (IntVal::I64($a), IntVal::I64($b)) => $expr,
            (IntVal::I128($a), IntVal::I128($b)) => $expr,
            (IntVal::ISize($a), IntVal::ISize($b)) => $expr,
            (IntVal::U8($a), IntVal::U8($b)) => $expr,
            (IntVal::U16($a), IntVal::U16($b)) => $expr,
            (IntVal::U32($a), IntVal::U32($b)) => $expr,
            (IntVal::U64($a), IntVal::U64($b)) => $expr,
            (IntVal::U128($a), IntVal::U128($b)) => $expr,
            (IntVal::USize($a), IntVal::USize($b)) => $expr,
            _ => panic!(""),
        }
    };
}

#[macro_export]
macro_rules! int_expr_tripple {
    ($a_expr:expr,$b_expr:expr,$c_expr:expr,$expr:expr,$a:ident,$b:ident,$c:ident) => {
        match ($a_expr,$b_expr,$c_expr) {
            (IntVal::I8($a), IntVal::I8($b), IntVal::I8($c)) => $expr,
            (IntVal::I16($a), IntVal::I16($b), IntVal::I16($c)) => $expr,
            (IntVal::I32($a), IntVal::I32($b), IntVal::I32($c)) => $expr,
            (IntVal::I64($a), IntVal::I64($b), IntVal::I64($c)) => $expr,
            (IntVal::I128($a), IntVal::I128($b), IntVal::I128($c)) => $expr,
            (IntVal::ISize($a), IntVal::ISize($b), IntVal::ISize($c)) => $expr,
            (IntVal::U8($a), IntVal::U8($b), IntVal::U8($c)) => $expr,
            (IntVal::U16($a), IntVal::U16($b), IntVal::U16($c)) => $expr,
            (IntVal::U32($a), IntVal::U32($b), IntVal::U32($c)) => $expr,
            (IntVal::U64($a), IntVal::U64($b), IntVal::U64($c)) => $expr,
            (IntVal::U128($a), IntVal::U128($b), IntVal::U128($c)) => $expr,
            (IntVal::USize($a), IntVal::USize($b), IntVal::USize($c)) => $expr,
            _ => panic!(""),
        }
    };
}
#[macro_export]
macro_rules! int_expr_single {
    ($lhs:expr,$expr:expr,$a:ident) => {
        match $lhs {
            IntVal::I8($a) => $expr,
            IntVal::I16($a) => $expr,
            IntVal::I32($a) => $expr,
            IntVal::I64($a) => $expr,
            IntVal::I128($a) => $expr,
            IntVal::ISize($a) => $expr,
            IntVal::U8($a) => $expr,
            IntVal::U16($a) => $expr,
            IntVal::U32($a) => $expr,
            IntVal::U64($a) => $expr,
            IntVal::U128($a) => $expr,
            IntVal::USize($a) => $expr,
        }
    };
}
#[macro_export]
macro_rules! int_expr_single_else {
    ($lhs:expr,$expr:expr,$expr2:expr,$a:ident) => {
        match $lhs {
            IntVal::I8($a) => $expr,
            IntVal::I16($a) => $expr,
            IntVal::I32($a) => $expr,
            IntVal::I64($a) => $expr,
            IntVal::I128($a) => $expr,
            IntVal::ISize($a) => $expr,
            IntVal::U8($a) => $expr2,
            IntVal::U16($a) => $expr2,
            IntVal::U32($a) => $expr2,
            IntVal::U64($a) => $expr2,
            IntVal::U128($a) => $expr2,
            IntVal::USize($a) => $expr2,
        }
    };
}

#[macro_export]
macro_rules! int_expr_single_else_signed {
    ($lhs:expr,$expr:expr,$expr2:expr,$a:ident) => {
        match $lhs {
            IntVal::I8($a) => $expr,
            IntVal::I16($a) => $expr,
            IntVal::I32($a) => $expr,
            IntVal::I64($a) => $expr,
            IntVal::I128($a) => $expr,
            IntVal::ISize($a) => $expr,
            _ => $expr2,
        }
    };
}

#[macro_export]
macro_rules! int_expr_single_else_unsigned {
    ($lhs:expr,$expr:expr,$expr2:expr,$a:ident) => {
        match $lhs {
            IntVal::U8($a) => $expr,
            IntVal::U16($a) => $expr,
            IntVal::U32($a) => $expr,
            IntVal::U64($a) => $expr,
            IntVal::U128($a) => $expr,
            IntVal::USize($a) => $expr,
            _ => $expr2
        }
    };
}


#[macro_export]
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