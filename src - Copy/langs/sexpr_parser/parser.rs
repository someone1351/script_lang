use std::path::Path;

use super::super::super::common::*;

use super::container::*;
use super::lexer::*;
use super::error::*;

pub fn parse(src : &str, keep_src : bool, path : Option<&Path>) -> Result<SExprTree,ParserError> {

    let mut lexer = Lexer::new(src.chars());
    let mut brace_stk =  Vec::<TokenBracketType>::new();
    let mut parent_stk = vec![0 as usize];//Vec::<usize>::new();
    let mut sexprs = vec![SExpr{
        val:SExprVal::List(Vec::new()),
        start_loc:Loc::default(),
        end_loc:Loc::default(),
        depth:0,
        child_ind:0,
    }];//Vec::<SExpr>::new();

    let mut last_loc= None;

    while let Some(token)=lexer.next()? {
        last_loc= Some(token.end_loc);
        let parent_sexpr_ind=*parent_stk.last().unwrap();

        let sexpr_val=match token.token_type {
            TokenType::LBracket(t)=>{
                brace_stk.push(t);
                // sexprs.push(SExpr::List(Vec::new()));
                parent_stk.push(sexprs.len());
                Some(SExprVal::List(Vec::new()))
            }
            TokenType::RBracket(t)=>{
                if let Some(b)=brace_stk.pop() {
                    if b==t {
                        parent_stk.pop();
                        sexprs[parent_sexpr_ind].end_loc=token.end_loc;
                    } else {
                        return Err(ParserError { loc: token.start_loc, error_type: ParserErrorType::MismatchedBraceTypes });
                    }
                } else {
                    return Err(ParserError { loc: token.start_loc, error_type: ParserErrorType::NoMatchingOpenForClosingBrace });
                }
                None
            }
            TokenType::Symbol(s) if s=="true" => Some(SExprVal::Bool(true)),
            TokenType::Symbol(s) if s=="false" => Some(SExprVal::Bool(false)),
            TokenType::Symbol(s)=>Some(SExprVal::Symbol(s)),
            TokenType::String(s)=>Some(SExprVal::String(s)),
            TokenType::Float(s)=>Some(SExprVal::Float(s.parse().unwrap())),
            TokenType::Int(s)=>Some(SExprVal::Int(s.parse().unwrap())),

        };
        if let Some(sexpr_val)=sexpr_val {

            let child_sexpr_ind = sexprs.len();

        
            let parent=sexprs.get_mut(parent_sexpr_ind).unwrap();

            let child_ind=if let SExprVal::List(child_sexpr_inds)=&parent.val {child_sexpr_inds.len()}else{0};

            if let SExprVal::List(child_sexpr_inds)=&mut parent.val {
                child_sexpr_inds.push(child_sexpr_ind);
            };
            
            let sexpr = SExpr{
                val:sexpr_val,
                start_loc:token.start_loc,
                end_loc:token.end_loc,
                depth : parent.depth+1,
                child_ind
            };

            sexprs.push(sexpr);
        }
        // println!("{:?}",token.token_type);
    }

    if brace_stk.len()>0 {
        return Err(ParserError { loc: last_loc.unwrap(), error_type: ParserErrorType::ExpectedClosingBrace });
    }

    //
    if let Some(x)=sexprs.get(1) {
        sexprs[0].start_loc=x.start_loc;        
    }

    if let Some(x)=sexprs.last() {
        sexprs[0].start_loc=x.end_loc;        
    }

    Ok(SExprTree::new(sexprs,if keep_src{Some(src)}else{None},path))
    // Ok(Ast {
    //     sexprs,
    //     src:if keep_src{Some(src.to_string())}else{None},
    //     path:path.and_then(|x|Some(x.to_path_buf())),
    // })

}
