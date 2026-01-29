
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

pub fn include_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() != 2 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    let v = record.param(1).unwrap().as_primitive();

    let Some(s)=v.as_string() else {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::ExpectString));
    };

    builder.include(s, v.start_loc());

    Ok(())
}
