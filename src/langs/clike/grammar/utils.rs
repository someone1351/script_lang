use std::fmt::Debug;
use crate::clike::grammar::{container::WalkGroupContainer, TokensTrait};

// pub fn groups_debug_print<'g, GR, TS>(root:WalkGroupContainer<'g, GR, TS>)
// where
//     TS:Clone+Iterator,
//     GR:Clone+Debug,
//     <TS as Iterator>::Item: Debug,
// {


//     let mut stk=vec![(root,0)];

//     while let Some((cur,depth))=stk.pop() {
//         let indent="    ".repeat(depth);
//         // let xx= cur.tokens().map(|x|format!("{x:?}")).collect::<Vec<_>>().join(", ");
//         // println!("{indent}group: {:?} : [{}]",cur.name2(),xx);
//         if let Some(name)=cur.name2() {
//             // let xx= cur.tokens().map(|x|format!("{x:?}")).collect::<Vec<_>>().join(", ");
//             println!("{indent}group: {name:?}",);
//             stk.extend(cur.children().rev().map(|c|(c,depth+1)));

//             if cur.children().len()==0 {
//                 for t in cur.tokens() {
//                     println!("{indent}    {t:?}");
//                 }
//             }
//         } else  {
//             for t in cur.tokens() {
//                 println!("{indent}{t:?}");
//             }

//         }
//     }
// }


pub fn groups_debug_print<'g, GR, P, TS>(root:WalkGroupContainer<'g, GR, TS>)
where
    TS:Clone+Iterator+TokensTrait<P>+DoubleEndedIterator,
    GR:Clone+Debug,
    <TS as Iterator>::Item: Debug,
{
    enum Thing<'g,GR,TS,T>
    where
        TS:Clone,
        T:Debug,
    {
        Token(T),
        Group(WalkGroupContainer<'g,GR,TS>),
    }

    let mut stk = vec![(Thing::Group(root),0)];

    while let Some((cur,depth))=stk.pop() {
        let indent="    ".repeat(depth);

        match cur {
            Thing::Group(cur) => {
                println!("{indent}group: {}",
                    cur.name2().map(|x|format!("{x:?}")).unwrap_or_else(||"_".to_string()),
                );

                let mut cur_tokens = cur.tokens();

                for child_group in cur.children().rev() {
                    let child_tokens=child_group.tokens();
                    let ps_len=cur_tokens.inds2().end-child_tokens.inds2().end;
                    let ps=cur_tokens.take2(ps_len).unwrap();

                    stk.extend(ps.map(|t|(Thing::Token(t),depth+1)).rev());
                    stk.push((Thing::Group(child_group),depth+1));
                    cur_tokens.take2(child_tokens.len2()).unwrap();
                }

                //
                stk.extend(cur_tokens.map(|t|(Thing::Token(t),depth+1)).rev());
            }
            Thing::Token(cur) => {
                println!("{indent}{cur:?}");
            }
        }
    }
}