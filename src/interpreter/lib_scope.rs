
// use std::any::Any;
// use std::cell::Cell;
// use std::cell::RefCell;
use std::collections::HashMap;
// use std::collections::HashSet;
// use std::marker::PhantomData;
// use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

// use parking_lot::Mutex;

// use super::super::common::*;
use super::libs;
// use super::var_scope::*;
use super::value::*;
use super::error::*;
use super::func_context::*;
// use super::machine::*;
use super::data::*;

/*
* have option to use method as get_field or not ?

TODO
* add num() ?
* allow specific float/int types eg .int_type::<i32>()

* allow i32 custom to be read as int param
*/


#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
pub enum Arg {
    Bool,
    Float,
    Int,
    String,

    Nil,
    // Func,

    Custom(std::any::TypeId),
    CustomMut(std::any::TypeId),
    CustomRef(std::any::TypeId),
    CustomMutRef(std::any::TypeId),

    CustomAny,
    CustomAnyMut,
    CustomAnyRef,
    CustomAnyMutRef,

    Any,
}

impl Arg {
    pub fn custom<T:'static>() -> Arg {
        Arg::Custom(std::any::TypeId::of::<T>())
    }
    pub fn custom_ref<T:'static>() -> Arg {
        Arg::CustomRef(std::any::TypeId::of::<T>())
    }
    pub fn custom_mut<T:'static>() -> Arg {
        Arg::CustomMut(std::any::TypeId::of::<T>())
    }
    pub fn custom_mut_ref<T:'static>() -> Arg {
        Arg::CustomMutRef(std::any::TypeId::of::<T>())
    }

    pub fn from_value(value:&Value) -> Option<Arg> {
        match value {
            Value::Bool(_) => Some(Arg::Bool),
            Value::Int(_) => Some(Arg::Int),
            Value::Float(_) => Some(Arg::Float),
            Value::String(_) => Some(Arg::String),
            Value::Custom(_) => None, //Some(Arg::Custom(c.type_info().id())), //why is this none? because don't know if it should be arg custom or customref, maybe default to custom?
            Value::Nil => Some(Arg::Nil),
            Value::Void => None,
            Value::Undefined => None,
        }
    }
    pub fn from_custom_value(value:&Value) -> Option<Arg> {
        if let Value::Custom(c)=value {
            // Some(Arg::Custom(c.type_id()))
            Some(Arg::Custom(c.type_info().id()))
        } else {
            None
        }
    }

    pub fn from_custom_value_ref(value:&Value) -> Option<Arg> {
        if let Value::Custom(c)=value {
            // Some(Arg::CustomRef(c.type_id()))
            Some(Arg::CustomRef(c.type_info().id()))
        } else {
            None
        }
    }

    pub fn from_custom_value_mut(value:&Value) -> Option<Arg> {
        value.get_custom().and_then(|c|c.is_mut().then(||Arg::CustomMut(c.type_info().id())))
    }
    pub fn from_custom_value_mut_ref(value:&Value) -> Option<Arg> {
        value.get_custom().and_then(|c|c.is_mut().then(||Arg::CustomMutRef(c.type_info().id())))
    }

    pub fn is_value(&self,value:&Value) -> bool {
        match (value,self) {
            (Value::Bool(_),Arg::Bool) => true,
            (Value::Int(_),Arg::Int) => true,
            (Value::Float(_),Arg::Float) => true,
            (Value::String(_),Arg::String) => true,

            // (Value::Custom(c),Arg::Func) => c.type_id()==std::any::TypeId::of::<Closure>(),

            (Value::Custom(c),Arg::Custom(t)) => c.type_info().id()==*t,
            (Value::Custom(c),Arg::CustomRef(t)) => c.type_info().id()==*t,

            (Value::Custom(_),Arg::CustomAny) => true,
            (Value::Custom(_),Arg::CustomAnyRef) => true,

            (Value::Custom(c),Arg::CustomMut(t)) => c.is_mut() && c.type_info().id()==*t,
            (Value::Custom(c),Arg::CustomMutRef(t)) => c.is_mut() && c.type_info().id()==*t,

            (Value::Custom(c),Arg::CustomAnyMut) => c.is_mut(),
            (Value::Custom(c),Arg::CustomAnyMutRef) => c.is_mut(),

            (_,Arg::Any) => true,

            _ => false,
        }
    }

    pub fn is_custom_not_ref(&self) -> bool {
        match self {
            Arg::Custom(_)|Arg::CustomAny| Arg::CustomMut(_)|Arg::CustomAnyMut => true,
            _ => false
        }
    }
    pub fn is_custom_ref(&self) -> bool {
        match self {
            Arg::CustomRef(_)|Arg::CustomAnyRef |Arg::CustomMutRef(_)|Arg::CustomAnyMutRef => true,
            _ => false
        }
    }
}

#[derive(Clone, Copy)]
pub enum MethodInputType<'m> {
    Method{name:&'m str},
    FieldNamed{name:&'m str},
    Field{no_symbols:bool},
}
#[derive(Clone, Hash,Eq,PartialEq )]
pub enum MethodInputType2 {
    Method{name:String},
    FieldNamed{name:String},
    Field{no_symbols:bool},
}
impl Into<MethodInputType2> for  MethodInputType<'_> {
    fn into(self) -> MethodInputType2 {
        match self {
            MethodInputType::Method { name } => MethodInputType2::Method { name: name.to_string() },
            MethodInputType::FieldNamed { name } => MethodInputType2::FieldNamed { name: name.to_string() },
            MethodInputType::Field { no_symbols: allow_symbols } => MethodInputType2::Field { no_symbols: allow_symbols },
        }
    }
}
// impl<'m> Into<MethodInputType<'m>> for MethodInputType2 {
//     fn into(self) -> MethodInputType<'m> {
//         match self {
//             MethodInputType2::Method { name } => MethodInputType::Method { name: name.as_str() },
//             MethodInputType2::FieldNamed { name } => MethodInputType::FieldNamed { name: name.as_str() },
//             MethodInputType2::Field { allow_symbols } => MethodInputType::Field { allow_symbols:allow_symbols },
//         }
//     }
// }

// impl<'m> MethodInputType2 {
//     fn my_to_ref(&'m self) -> MethodInputType<'m> {
//         match self {
//             MethodInputType2::Method { name } => MethodInputType::Method { name: name.as_str() },
//             MethodInputType2::FieldNamed { name } => MethodInputType::FieldNamed { name: name.as_str() },
//             MethodInputType2::Field { allow_symbols } => MethodInputType::Field { allow_symbols:*allow_symbols },
//         }
//     }
// }
// impl<'m> std::borrow::Borrow<MethodInputType<'m>> for MethodInputType2 {
//     fn borrow(& self) -> &'m MethodInputType<'m> {
//        &self.my_to_ref()
//     }
// }
pub struct MethodInput<'m,X> {
    lib_scope:&'m mut LibScope<X>,
    // name:&'m str,
    input_type:MethodInputType<'m>,
    method_type:MethodType<X>,

    args : Vec<Vec<Arg>>,
    optional_start : Option<usize>,
    // variadic:bool,
}

impl<'m,X> MethodInput<'m,X> {
    pub fn optional(mut self) -> Self {
        self.optional_start=Some(self.args.len());
        // println!("opt {:?}",self.optional_start);
        self
    }
    pub fn bool(mut self) -> Self {
        self.args.push(vec![Arg::Bool]);
        self
    }
    pub fn int(mut self) -> Self {
        self.args.push(vec![Arg::Int]);
        self
    }
    pub fn float(mut self) -> Self {
        self.args.push(vec![Arg::Float]);
        self
    }
    pub fn str(mut self) -> Self {
        self.args.push(vec![Arg::String]);
        self
    }
    pub fn nil(mut self) -> Self {
        self.args.push(vec![Arg::Nil]);
        self
    }
    pub fn any(mut self) -> Self {
        self.args.push(vec![Arg::Any]);
        self
    }
    pub fn custom_any(mut self) -> Self {
        self.args.push(vec![Arg::CustomAny]);
        self
    }
    pub fn custom_any_ref(mut self) -> Self {
        self.args.push(vec![Arg::CustomAnyRef]);
        self
    }
    pub fn func(mut self) -> Self {
        self.args.push(vec![Arg::custom::<Closure>()]);
        self
    }
    pub fn custom<T:'static>(mut self) -> Self {
        self.args.push(vec![Arg::custom::<T>()]);
        self
    }
    pub fn custom_ref<T:'static>(mut self) -> Self {
        self.args.push(vec![Arg::custom_ref::<T>()]);
        self
    }


    pub fn custom_any_mut(mut self) -> Self {
        self.args.push(vec![Arg::CustomAnyMut]);
        self
    }
    pub fn custom_any_mut_ref(mut self) -> Self {
        self.args.push(vec![Arg::CustomAnyMutRef]);
        self
    }
    pub fn custom_mut<T:'static>(mut self) -> Self {
        self.args.push(vec![Arg::custom_mut::<T>()]);
        self
    }
    pub fn custom_mut_ref<T:'static>(mut self) -> Self {
        self.args.push(vec![Arg::custom_mut_ref::<T>()]);
        self
    }

    pub fn or_bool(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::Bool);
        self
    }
    pub fn or_int(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::Int);
        self
    }
    pub fn or_float(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::Float);
        self
    }
    pub fn or_str(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::String);
        self
    }
    pub fn or_nil(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::Nil);
        self
    }
    pub fn or_any(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::Any);
        self
    }
    pub fn or_custom_any(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::CustomAny);
        self
    }
    pub fn or_custom_any_ref(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::CustomAnyRef);
        self
    }
    pub fn or_func(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::custom::<Closure>());
        self
    }
    pub fn or_custom<T:'static>(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::custom::<T>());
        self
    }
    pub fn or_custom_ref<T:'static>(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::custom_ref::<T>());
        self
    }

    pub fn or_custom_any_mut(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::CustomAnyMut);
        self
    }
    pub fn or_custom_any_mut_ref(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::CustomAnyMutRef);
        self
    }
    pub fn or_custom_mut<T:'static>(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::custom_mut::<T>());
        self
    }
    pub fn or_custom_mut_ref<T:'static>(mut self) -> Self {
        self.args.last_mut().unwrap().push(Arg::custom_mut_ref::<T>());
        self
    }

    pub fn inner_end(mut self,variadic:bool) -> Self {
        if self.args.len() > 0 {

                //could n= arg0.len*arg1.len * .. * argn.len
                // for i in 0 ...n { convert i to positions }

                //let mut i=0;

                //i = y * w + x
                //x = i % w;
                //y = i / w;


                //i=(z * w * h) + (y * w) + x;
                //i=x + (y + (z * h))*w;
                //z = i / (w * h);
                //y = (i-(z * w * h)) / w;
                //x = (i-(z * w * h)) % w;


                //i = x + y * w + z * w * h
                //x = i % w
                //y = ( i / w ) % h
                //z = i / ( w * h )



            //
            let mut positions = vec![0;self.args.len()];

            loop {
                for i in 0 .. self.args.len()-1 {
                    if positions[i]==self.args[i].len() {
                        positions[i]=0;
                        *positions.get_mut(i+1).unwrap() +=1;
                    } else {
                        break;
                    }
                }

                if *positions.last().unwrap() == self.args.last().unwrap().len() {
                    break;
                }

                // println!("{:?}",positions.iter().map(|x|x.to_string()).collect::<String>());

                let args = positions.iter().enumerate().map(|(arg_ind,&x)|self.args
                    .get(arg_ind).unwrap().get(x).unwrap().clone()).collect::<Vec<_>>();

                // println!("args {:?}, {:?}",args,self.optional_start);

                // match self.input_type {
                //     MethodInputType::Method { name } => {
                //         self.lib_scope.inner_insert_method(name, args, self.optional_start, variadic, self.method_type.clone());

                //     }
                //     MethodInputType::FieldNamed { name } => {

                //     }
                //     MethodInputType::Field { allow_symbols } => {

                //     }
                // }
                self.lib_scope.inner_insert_method(self.input_type, args, self.optional_start, variadic, self.method_type.clone());

                positions[0]+=1;
            }
        } else {

            // match self.input_type {
            //     MethodInputType::Method { name } => {
            //         self.lib_scope.inner_insert_method(name, [], self.optional_start, variadic, self.method_type.clone());

            //     }
            //     MethodInputType::FieldNamed { .. } => {
            //     }
            //     MethodInputType::Field { .. } => {
            //     }
            // }

            self.lib_scope.inner_insert_method(self.input_type, [], self.optional_start, variadic, self.method_type.clone());
        }

        //
        self.args.clear();
        self.optional_start=None;
        self
    }

    pub fn end(self) -> Self {
        self.inner_end(false)
    }
    pub fn variadic_end(self) -> Self {
        self.inner_end(true)
    }
}

pub type FuncType<X> = Arc<dyn Fn(FuncContext<X>)->Result<Value,MachineError>+'static +Send+Sync> ;
pub type FuncTypeMut<X> = Arc<Mutex<dyn FnMut(FuncContext<X>)->Result<Value,MachineError> + 'static + Send + Sync>>;

#[derive(Clone,)] //Default
pub struct ArgNode<X> {
    // pub arg_type : ValueType,
    pub variadic : bool,
    // pub optional : bool,
    pub children : HashMap<Arg,usize>,
    func:Option<MethodType<X>>,
}
impl<X> Default for ArgNode<X> {
    fn default() -> Self {
        Self { variadic: false, children: HashMap::new(), func: None }
    }
}
impl<X> std::fmt::Debug for ArgNode<X> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ArgNode(variadic : {}, children : {:?}, func : {})",
            self.variadic,
            self.children,
            self.func.is_some(),
        )
    }
}

// #[derive(Clone)]
pub enum MethodType<X> {
    Mut(FuncTypeMut<X>),
    NonMut(FuncType<X>),
}

impl<X> Clone for MethodType<X> {
    fn clone(&self) -> Self {
        match self {
            Self::Mut(x) => Self::Mut(x.clone()),
            Self::NonMut(x) => Self::NonMut(x.clone()),
        }
    }
}

#[derive(Clone)]
pub struct Method<X> {
    pub method_type:MethodType<X>,
    pub args_path:Vec<Arg>,
}


#[derive(Clone)]
pub struct LibScope<X> { //
    constants : HashMap<String,Value>,
    // methods : HashMap<String,usize>, //node_ind
    methods : HashMap<MethodInputType2,usize>, //node_ind
    nodes : Vec<ArgNode<X>>,
}


impl<X> Default for LibScope<X> {
    fn default() -> Self {
        Self::new_full()
    }
}

impl<X> LibScope<X> {
    pub fn new() -> Self {
        Self {
            constants: Default::default(),
            methods: Default::default(),
            nodes: Default::default(),
            // p:Default::default(),
        }
    }
    pub fn new_full() -> Self {
        let mut lib_scope = Self::new();
        libs::register_all(&mut lib_scope);
        lib_scope
    }

    pub fn insert_constant(&mut self,n:&str,v:Value) {
        self.constants.insert(n.to_string(), v.clone_root());
    }

    pub fn get_constant(&self,n : &str) -> Option<Value> {
        self.constants.get(&n.to_string()).map(|x|x.clone_root())
    }

    fn get_insert_root_node_ind(&mut self,
        // n:&str
        input_type:MethodInputType<'_>,
    ) -> usize {
        *self.methods.entry(
            // n.to_string()
            input_type.into()
        ).or_insert_with(||{
            let ind=self.nodes.len();
            self.nodes.push(Default::default());
            ind
        })
    }

    fn get_insert_child_node_ind(&mut self, node_ind:usize, value_type : Arg) -> usize {
        if let Some(&child_node_ind)=self.get_node(node_ind).children.get(&value_type) {
            child_node_ind
        } else {
            let child_node_ind=self.nodes.len();
            self.get_node_mut(node_ind).children.insert(value_type, child_node_ind);
            self.nodes.push(Default::default());
            child_node_ind
        }
    }

    fn get_root_node_ind(&self,
        // n:&str
        input_type:&MethodInputType2
    ) -> Option<usize> {
        self.methods.get(input_type).cloned()
    }

    fn get_node(&self,node_ind:usize) -> &ArgNode<X> {
        self.nodes.get(node_ind).unwrap()
    }

    fn get_node_mut(&mut self,node_ind:usize) -> &mut ArgNode<X> {
        self.nodes.get_mut(node_ind).unwrap()
    }

    fn get_child_node_ind(&self, node_ind:usize, value_type : Arg) -> Option<usize> {
        self.get_node(node_ind).children.get(&value_type).cloned()
    }

    fn inner_insert_method<T>(&mut self,
        // n:&str,
        input_type:MethodInputType<'_>,
        args : T,optional_start : Option<usize>,variadic:bool,func:MethodType<X>)
    where
        T:AsRef<[Arg]>
    {
        let args=args.as_ref().to_vec();
        let root_node_ind=self.get_insert_root_node_ind(input_type //n
            );

        if args.len()==0 || optional_start==Some(0) {
            let root_node=self.nodes.get_mut(root_node_ind).unwrap();
            root_node.func=Some(func.clone());
        }

        //
        let mut cur_node_ind=root_node_ind;

        for (arg_ind,&arg) in args.iter().enumerate() {
            let child_node_ind=self.get_insert_child_node_ind(cur_node_ind, arg);
            let child_node=self.get_node_mut(child_node_ind);

            //
            let is_next_optional=optional_start.is_some_and(|optional_start|arg_ind+1>=optional_start);
            let is_end = arg_ind+1==args.len();
            let is_variadic = is_end && variadic;

            child_node.variadic=is_variadic;

            if is_end || is_next_optional {
                child_node.func=Some(func.clone());
            }

            cur_node_ind=child_node_ind;
        }
    }

    fn inner_get_method<'x,I>(&self,
        // n : &str,
        input_type:&MethodInputType2,
        params : I) -> Option<Method<X>>
    where
        I: IntoIterator<Item=&'x Value>,
    {
        // println!("methods {:?}",self.methods);
        // println!("nodes {:?}",self.nodes);

        let params = params.into_iter().collect::<Vec<_>>();

        let Some(root_node_ind)=self.get_root_node_ind(input_type) else {
            return None;
        };

        //
        if params.len()==0 {
            let root_node=self.get_node(root_node_ind);
            let func=root_node.func.clone();

            return func.and_then(|func|Some(Method{
                method_type:func,
                args_path:Vec::new(),
            }));
        }

        //
        let mut best_score : usize = 0;
        let mut best_func : Option<MethodType<X>> = None;
        let mut best_path : Vec<Arg> = Vec::new();

        //
        let mut stk: Vec<(usize, usize, usize,Vec<Arg>)>=vec![(0,root_node_ind,0,Vec::new())]; //(param_ind,node_ind,score,path)

        while let Some((param_ind,node_ind,score,path))=stk.pop() {
            let is_end=param_ind+1==params.len();
            let param=*params.get(param_ind).unwrap();
            // let node=self.get_node(node_ind);

            //
            let mut todos=Vec::new(); //param_type,node_ind,score

            //make list of children of cur node to traverse
            if param.is_custom_any() {
                // for arg_type in [
                //     Arg::from_custom_value(param),
                //     Arg::from_custom_value_ref(param),
                //     Arg::from_custom_value_mut(param),
                //     Arg::from_custom_value_mut_ref(param),
                // ] {
                //     let arg_type=arg_type.unwrap();

                //     if let Some(child_node_ind)=self.get_child_node_ind(node_ind, arg_type) {
                //         // let child_node=self.get_node(child_node_ind);
                //         todos.push((arg_type,child_node_ind,score+6)); //specific types have highest score
                //     }
                // }

                for arg_type in [Arg::from_custom_value(param),Arg::from_custom_value_ref(param),] {
                    let arg_type=arg_type.unwrap();

                    if let Some(child_node_ind)=self.get_child_node_ind(node_ind, arg_type) {
                        todos.push((arg_type,child_node_ind,score+6)); //specific types have highest score
                    }
                }

                if param.is_mut() {
                    for arg_type in [Arg::from_custom_value_mut(param),Arg::from_custom_value_mut_ref(param),] {
                        let arg_type=arg_type.unwrap();

                        if let Some(child_node_ind)=self.get_child_node_ind(node_ind, arg_type) {
                            todos.push((arg_type,child_node_ind,score+6)); //specific types have highest score
                        }
                    }
                }
            } else if let Some(arg_type)=Arg::from_value(param) {
                if let Some(child_node_ind)=self.get_child_node_ind(node_ind, arg_type) {
                    // let child_node=self.get_node(child_node_ind);
                    todos.push((arg_type,child_node_ind,score+6)); //specific types have highest score
                }
            }

            if param.is_custom_any() {
                if let Some(child_node_ind)=self.get_child_node_ind(node_ind, Arg::CustomAny) {
                    todos.push((Arg::CustomAny,child_node_ind,score+4)); //general custom lessor score
                }

                if let Some(child_node_ind)=self.get_child_node_ind(node_ind, Arg::CustomAnyRef) {
                    todos.push((Arg::CustomAnyRef,child_node_ind,score+4)); //general custom lessor score
                }

                if param.is_mut() {
                    if let Some(child_node_ind)=self.get_child_node_ind(node_ind, Arg::CustomAnyMut) {
                        todos.push((Arg::CustomAnyMut,child_node_ind,score+4)); //general custom lessor score
                    }


                    if let Some(child_node_ind)=self.get_child_node_ind(node_ind, Arg::CustomAnyMutRef) {
                        todos.push((Arg::CustomAnyMutRef,child_node_ind,score+4)); //general custom lessor score
                    }
                }
            }

            if let Some(child_node_ind)=self.get_child_node_ind(node_ind, Arg::Any) {
                todos.push((Arg::Any,child_node_ind,score+2)); //and dynamic the smallest score
            }

            //
            for (arg_type,child_node_ind,child_score) in todos {
                let child_node=self.get_node(child_node_ind);

                let mut path=path.clone();
                path.push(arg_type);

                if is_end {
                    if let Some(child_func)=&child_node.func {
                        if child_score>best_score {

                            best_score=child_score;
                            best_func=Some(child_func.clone());
                            best_path=path;
                        }
                    }
                } else {
                    if child_node.variadic {
                        let mut inner_child_score_sum = child_score;
                        let mut ok=true;

                        for inner_param_ind in param_ind+1 .. params.len() {
                            let inner_param=*params.get(inner_param_ind).unwrap();

                            // if !inner_param.is_type(arg_type)
                            if !arg_type.is_value(inner_param)
                            {
                                ok=false;
                                break;
                            }

                            inner_child_score_sum+=child_score-1; //variadic methods have lesser score to nonvariadic
                        }

                        if ok {
                            if inner_child_score_sum>best_score {
                                best_score=inner_child_score_sum;
                                best_func=Some(child_node.func.clone().unwrap());
                                best_path=path.clone();
                            }
                        }
                    }

                    //also add it to the stk, incase it has any children that can be used
                    stk.push((param_ind+1,child_node_ind,child_score,path));
                }
            }

        }

        return best_func.and_then(|func|Some(Method{
            method_type:func,
            args_path:best_path,
        }));
    }

    pub fn get_method<'x,I>(&self,
        n : &str,
        params : I,
        // var_scope : &VarScope,
    ) -> Option<Method<X>>
    where
        I: IntoIterator<Item=&'x Value>
    {
        self.inner_get_method(&MethodInputType2::Method { name: n.to_string() }, params)
    }

    pub fn get_method_field_named<'x,I>(&self,
        n : &str,
        params : I,
        // var_scope : &VarScope,
    ) -> Option<Method<X>>
    where
        I: IntoIterator<Item=&'x Value>
    {
        self.inner_get_method(&MethodInputType2::FieldNamed{ name: n.to_string() }, params)
    }

    pub fn get_method_field<'x,I>(&self,
        no_symbols:bool,
        params : I,
        // var_scope : &VarScope,
    ) -> Option<Method<X>>
    where
        I: IntoIterator<Item=&'x Value>
    {
        self.inner_get_method(&MethodInputType2::Field { no_symbols }, params)
    }

    pub fn field_named<'m>(&'m mut self,name : &'m str,
        func: impl Fn(FuncContext<X>)->Result<Value,MachineError>+'static+Send+Sync
    ) -> MethodInput<'m,X> {
        MethodInput {
            lib_scope: self,
            // name,
            input_type:MethodInputType::FieldNamed { name, },
            method_type:MethodType::NonMut(Arc::new(func)),
            args: Vec::new(),
            optional_start: None,
            // variadic: false,
        }
    }
    pub fn field_no_symbols<'m>(&'m mut self,
        func: impl Fn(FuncContext<X>)->Result<Value,MachineError>+'static+Send+Sync
    ) -> MethodInput<'m,X> {
        MethodInput {
            lib_scope: self,
            // name,
            input_type:MethodInputType::Field { no_symbols:true, } ,
            method_type:MethodType::NonMut(Arc::new(func)),
            args: Vec::new(),
            optional_start: None,
            // variadic: false,
        }
    }
    pub fn field<'m>(&'m mut self, //no_symbols:bool,
        func: impl Fn(FuncContext<X>)->Result<Value,MachineError>+'static+Send+Sync
    ) -> MethodInput<'m,X> {
        MethodInput {
            lib_scope: self,
            // name,
            input_type:MethodInputType::Field { no_symbols:false, } ,
            method_type:MethodType::NonMut(Arc::new(func)),
            args: Vec::new(),
            optional_start: None,
            // variadic: false,
        }
    }
    pub fn method<'m>(&'m mut self,name : &'m str,
        func: impl Fn(FuncContext<X>)->Result<Value,MachineError>+'static+Send+Sync
    ) -> MethodInput<'m,X> {
        MethodInput {
            lib_scope: self,
            // name,
            input_type:MethodInputType::Method { name, },
            method_type:MethodType::NonMut(Arc::new(func)),
            args: Vec::new(),
            optional_start: None,
            // variadic: false,
        }
    }
    // pub fn method_mut<'m>(&'m mut self,name : &'m str,
    //     // slot:usize
    //     func:impl FnMut(FuncContext<X>)->Result<Value,MachineError> + 'static + Send + Sync
    // ) -> MethodInput<'m,X> {
    //     MethodInput {
    //         lib_scope: self,
    //         name,
    //         method_type:MethodType::Mut(Arc::new(Mutex::new(func))),
    //         args: Vec::new(),
    //         optional_start: None,
    //         // variadic: false,
    //     }
    // }

    //


}

/*
current:
    lib_scope.method("method",|context|Ok(Value::Nil))
        .any().end()

idea 1:
    lib_scope
        .method("parent").custom_ref::<Entity>().end()
        .getter(Some("parent")).custom_ref::<Entity>().end()
        .submit(|context|{
            Ok(Value::Nil)
        });

    lib_scope
        .setter(Some("parent")).custom_ref::<Entity>().custom_ref::<Entity>().end()
        .submit(|context|{
            Ok(Value::Nil)
        });

idea2:
    let parent_bind=lib_scope.binding(|context|{
        Ok(Value::Nil)
    });

    parent_bind.method("parent").custom_ref::<Entity>().end();
    parent_bind.getter(Some("parent")).custom_ref::<Entity>().end();

    let set_parent_bind=lib_scope.binding(|context|{
        Ok(Value::Nil)
    });
    set_parent_bind.custom_ref::<Entity>().custom_ref::<Entity>().end()

*/


// impl<'b,'a:'b,X> LibScope<'a,X> {
//     pub fn make_copy(&self) -> LibScope<'b,X> {
//         let mut nodes: Vec<ArgNode<'b,X>> = Vec::new();

//         for node in self.nodes.iter() {
//             let func=match &node.func {
//                 Some(MethodType::NonMut(x)) => Some(MethodType::NonMut(x.clone())),
//                 Some(MethodType::Mut(x)) => Some(MethodType::Mut(x.clone())),
//                 None => None
//             };

//             nodes.push(ArgNode { variadic: node.variadic, children: node.children.clone(), func });
//         }

//         LibScope {
//             constants : self.constants.clone(),
//             methods : self.methods.clone(),
//             nodes,
//         }
//     }
// }