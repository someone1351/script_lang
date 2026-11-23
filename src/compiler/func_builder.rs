use std::{fmt::Debug, path::Path};

use crate::{ast::{error::AstVarError, Ast}, builder::{Builder, BuilderError}, BuildT, StringVal};

//google says so

#[derive(Debug, Clone)]
pub enum FuncCompileError<E: Clone + Debug> {
    Builder(BuilderError<E>),
    AstVar(AstVarError),
}

//google says so
pub fn compile_func_builder<'a, 'p, T, E, P, F>(
    // name: Option<&'a str>,
    params: P,
    variadic: bool,
    version: usize,
    path: Option<&'p Path>,
    src: Option<StringVal>,
    keep_locs: bool,
    build_fn: F,
    mut eval_cb: impl FnMut(&mut Builder<'a, T, E>, T) -> Result<(), BuilderError<E>> + 'a,
) -> Result<BuildT, FuncCompileError<E>>
where
    T: Clone + 'a,
    E: Clone + Debug + 'a,
    P: IntoIterator<Item = &'a str>,
    F: FnOnce(&mut Builder<'a, T, E>),
{
    let mut builder = Builder::new();
    builder.func_start(params, variadic, None);
    build_fn(&mut builder);
    builder.func_end().end_stack_check();

    let mut ast = Ast::new(false, true);
    builder
        .generate_ast(&mut ast, move|b, p| eval_cb(b, p))
        .map_err(FuncCompileError::Builder)?;
    ast.calc_vars(false).map_err(FuncCompileError::AstVar)?;
    ast.calc_labels_gotos().map_err(FuncCompileError::AstVar)?;

    let build = ast.compile(version, path, src, keep_locs, keep_locs);
    Ok(BuildT::new(build))
}


//google says so
pub fn compile_func<'a, P, F>(
    // name: Option<&'a str>,
    params: P,
    variadic: bool,
    build_fn: F,
) -> Result<BuildT, FuncCompileError<()>>
where
    P: IntoIterator<Item = &'a str>,
    F: FnOnce(&mut Builder<'a, (), ()>),
{
    compile_func_builder(params, variadic, 0, None, None, false, build_fn, |_, _| Ok(()))
}
