// use super::super::super::common::*;

// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;
// use super::utils::*;

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    //len(str)
    lib_scope.method("len", |context|{
        Ok(Value::int(context.param(0).as_string().chars().count()))
    })
        .str().end();

    //contains(str,str)
    lib_scope.method("contains", |context|{
        let string=context.param(0).as_string();
        let val=context.param(1).as_string();
        Ok(Value::Bool(string.contains(&val)))
    })
        .str().str().end();

    //is_string(str)
    lib_scope.method("is_string", |_|{
        Ok(Value::Bool(true))
    })
        .str().end();

    //is_string(any)
    lib_scope.method("is_string", |_|{
        Ok(Value::Bool(false))
    })
        .any().end();

    //add(str,str), add(any,str), add(str,any)
    lib_scope.method("+",|context|{
        let s0=context.param(0).as_string();
        let s1=context.param(1).as_string();
        Ok(Value::string(format!("{s0}{s1}")))
    })
        .str().or_any().str().end()
        .str().str().or_any().end();

    //eq(str,str), eq(any,str), eq(str,any)
    lib_scope.method("=",|context|{
        let s0=context.param(0).as_string();
        let s1=context.param(1).as_string();
        Ok(Value::Bool(s0.eq(&s1)))
    })
        .str().or_any().str().end()
        .str().str().or_any().end();

    //string(str)
    lib_scope.method("string",|context|{
        Ok(context.param(0).clone())
    })
        .str().end();

    //repeat(str,int)
    lib_scope.method("repeat",|context|{
        let s=context.param(0).as_string();
        let r=context.param(1).as_int() as usize;
        Ok(Value::string(s.repeat(r)))
    })
        .str().int().end();

    //substr(str,int,int?)
    lib_scope.method("substr",|context|{
        let string=context.param(0).as_string();

        let start=context.param(1).as_int().max(0) as usize;
        let start=start.min(string.len());

        let end=if context.params_num()==2{string.len()}else{context.param(2).as_int().max(0) as usize};
        let end=end.min(string.len());

        Ok(Value::string(if start>=end { "" } else { &string[start..end] }))
    })
        .str().int().optional().int().end();

}