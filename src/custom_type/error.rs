#[derive(Debug,Eq,PartialEq)]
pub enum CustomError {

    // CustomDataBorrowError,
    CustomDataBorrowMutError,
    // CustomInstanceEmpty,
    // CustomIdEmpty,
    // CustomOwnerIdEmpty,
    CustomDataDead,
    CustomDataEmpty,
    CustomDataNotMut,
    CustomDataNotNonMut,
    CustomDataInvalidCast{given_type:String,expecting_type:String,},

}


impl std::fmt::Display for CustomError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{self}",)
    }
}

impl std::error::Error for CustomError{
    fn description(&self) -> &str {
        "Custom Error"
    }
}
