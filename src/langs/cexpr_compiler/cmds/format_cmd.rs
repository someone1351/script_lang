

use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

pub fn format_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //format "hello {a} {}" b

    //
    builder
        .result_string("")
        .param_push();

    //
    let mut j=1;

    if let Some(s)=record.param(1).unwrap().as_primitive().as_string() //record.get(1).unwrap().is_string()
    {
        j+=1;


        //parse format string

        // let s =record.get(1).unwrap().string().unwrap();
        let mut cs=s.chars();
        let mut char_ind=0;

        let mut texts: Vec<(usize, usize)>= vec![(0,0)];
        let mut vars: Vec<(usize, usize)> =Vec::new();

        while let Some(c)=cs.next() {
            char_ind+=1;

            match c {
                '{' => {
                    vars.push((char_ind,char_ind));

                    while let Some(c)=cs.next() {
                        char_ind+=1;

                        match c {
                            '}' => {
                                vars.last_mut().unwrap().1=char_ind-1;
                                break;
                            }
                            '\\' => {
                                if cs.next().is_some() {
                                    char_ind+=1;
                                }

                                continue;
                            }
                            _=>{
                            }
                        }
                    }

                    texts.push((char_ind,char_ind));
                }
                '\\' => {
                    if cs.next().is_some() {
                        char_ind+=1;
                        texts.last_mut().unwrap().1=char_ind;
                    }

                    continue;
                }
                _=>{
                    texts.last_mut().unwrap().1=char_ind;
                }
            }
        }

        //

        for i in 0 .. texts.len() {
            let (text_start,text_end) = texts[i];
            let text_str=&s[text_start..text_end];

            // println!("a: {text_str:?}");

            if !text_str.is_empty() {
                builder
                    .result_string(text_str)
                    .param_push()
                    .swap()
                    .call_method("+", 2)
                    .param_push()
                    ;
            }

            if let Some((var_start,var_end)) = vars.get(i).cloned() {
                let var_str=&s[var_start..var_end];
                // println!("v: {var_str:?}");

                if var_str.is_empty() {
                    // println!("hmm2 {:?}",record.start_loc());
                    // println!("hmm {:?}",record.param(j).map(|x|x.primitive()));

                    if let Some(x)=record.param(j) {
                        builder.eval(x.as_primitive());
                    } else {
                        builder.result_nil();
                    }

                    j+=1;
                } else {
                    builder.get_var(var_str);
                }

                builder
                    .param_push()
                    .call_method("string", 1)

                    .param_push()
                    .swap()
                    .call_method("+", 2)

                    .param_push();
            }
        }

    }

    for k in j .. record.params_num() {
        if k>=2 {
            builder
                .result_string(" ")

                .param_push()
                .swap()
                .call_method("+", 2)
                .param_push();
        }


        builder

            .eval(record.param(k).unwrap().as_primitive())

            .param_push()
            .call_method("string", 1)

            .param_push()
            .swap()
            .call_method("+", 2)

            .param_push();
    }


    builder.pop();
    // builder.call_method("console_out", 1);
//
    return Ok(());

}