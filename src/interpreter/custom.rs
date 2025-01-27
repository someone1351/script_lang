/*
if custom is root and is cloned, it will lose its root status, need to use clone_root() to keep it
might be better for clone to keep the root status, and when passed as param, then convert to not root

in machine will need to replace clone() with clone_not_root()
*/

// use parking_lot::{Mutex,MutexGuard,MappedMutexGuard};

use std::any::Any;
use std::sync::Arc;
// use std::sync::MappedMutexGuard;
use std::sync::Mutex;
// use std::sync::MutexGuard;

use super::gc_scope::*;
// use super::value::*;
use super::error::*;

// pub type Caller = fn(super::FuncContext)->Result<super::Value,MachineError>;

#[derive(Clone)]
pub enum StrongValueInner {
    Mut(Arc<Mutex<dyn Any+Send>>),
    NonMut(Arc<dyn Any+Send+Sync>),
    
    // MutExt(Arc<Mutex<dyn ToString+Send>>),
    // NonMutExt(Arc<dyn ToString+Send+Sync>),
}

impl StrongValueInner {
    // pub fn get_non_mut(&self) -> Option<Arc<dyn Any+Send+Sync>> {
    //     match self {
    //         Self::NonMut(data) =>Some(data.clone()),
    //         Self::NonMutExt(data) =>Some(),
    //         _ => None,
    //     }
    // }
    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::Mut(_) => None,
            Self::NonMut(_) => None,
            // Self::MutExt(x) => x.try_lock().map(|y|y.to_string()),
            // Self::NonMutExt(x)=>Some(x.to_string()),
        }
    }
}

#[derive(Clone)]
pub enum CustomInner {
    Managed(GcValue),   
    // Unmanaged(Arc<Mutex<dyn Any+Send>>),
    Unmanaged(StrongValueInner),
    // UnmanagedStatic(Arc<dyn Any+Send>),
    // None(&'static str),
}

impl CustomInner {
    pub fn clone_root(&self) -> Self {
        match self {
            Self::Managed(x)=>Self::Managed(x.clone_root()),
            Self::Unmanaged(x)=>Self::Unmanaged(x.clone()),
            // Self::UnmanagedStatic(x) => Self::UnmanagedStatic(x.clone()),
            // Self::None(x) => Self::None(*x),
        }
    }
    pub fn clone_as_is(&self) -> Self {
        match self {
            Self::Managed(x)=>Self::Managed(x.clone_as_is()),
            Self::Unmanaged(x)=>Self::Unmanaged(x.clone()),
            // Self::UnmanagedStatic(x) => Self::UnmanagedStatic(x.clone()),
        }
    }
}

#[derive(Clone)]

pub struct Custom {
    // type_id : std::any::TypeId,
    // type_name : &'static str,
    type_info:TypeInfo,
    inner : CustomInner,
    // caller : Option<Caller>,
}


impl std::string::ToString for Custom {
    fn to_string(&self) -> String {
        self.data().get_string().unwrap_or_else(||self.type_info.short_name())        
    }
}

impl std::fmt::Debug for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(gc_index)=self.gc_index() {
            write!(f, "Managed:{gc_index}({})",self.type_info.short_name())
        } else {
            write!(f, "Unmanaged({})",self.type_info.short_name())
        }        
    }
}

impl Custom {
    fn new<T:Any>(inner:CustomInner,
        // caller : Option<Caller>
    ) -> Self {
        Self {
            type_info:TypeInfo::new::<T>(), 
            inner,
            // caller,
        }
    }

    pub fn new_managed_mut<T:GcTraversable+Send>(data : T, 
        // caller:Option<Caller>, 
        gc_scope : &mut GcScope) -> Self 
    {
        Self::new::<T>(CustomInner::Managed(GcValue::new(data,gc_scope)),
            // caller
        )
    }
    
    pub fn new_unmanaged_mut<T:Any+Send>(data : T, //caller:Option<Caller>
        ) -> Self {
        // Self::new::<T>(CustomInner::Unmanaged(Arc::new(Mutex::new(data))))

        Self::new::<T>(CustomInner::Unmanaged(StrongValueInner::Mut(Arc::new(Mutex::new(data)))),//caller
            )
        
    }

    pub fn new_managed<T:GcTraversable+Send+Sync>(data : T, //caller:Option<Caller>,
        gc_scope : &mut GcScope) -> Self {
        Self::new::<T>(CustomInner::Managed(GcValue::new_non_mut(data,gc_scope)),//caller
        )
    }

    pub fn new_unmanaged<T:Any+Send+Sync>(data : T, //caller:Option<Caller>
        ) -> Self {
        Self::new::<T>(CustomInner::Unmanaged(StrongValueInner::NonMut(Arc::new(data))),//caller
        )        
    }



    // pub fn new_managed_mut_ext<T:GcTraversable+Send+ToString>(data : T,gc_scope : &mut GcScope) -> Self {
    //     Self::new::<T>(CustomInner::Managed(GcValue::new(data,gc_scope)))
    // }
    
    // pub fn new_unmanaged_mut_ext<T:Any+Send+ToString>(data : T) -> Self {
    //     Self::new::<T>(CustomInner::Unmanaged(StrongValueInner::MutExt(Arc::new(Mutex::new(data)))))
    // }

    // pub fn new_managed_non_mut_ext<T:GcTraversableExt+Send+Sync>(data : T,gc_scope : &mut GcScope) -> Self {
    //     Self::new::<T>(CustomInner::Managed(GcValue::new_non_mut_ext(data,gc_scope)))
    // }

    // pub fn new_unmanaged_non_mut_ext<T:Any+ToString+Send+Sync>(data : T) -> Self {
    //     Self::new::<T>(CustomInner::Unmanaged(StrongValueInner::NonMutExt(Arc::new(data))))        
    // }

    // pub fn get_caller(&self) -> Option<Caller> {
    //     return self.caller
    // }

    pub fn clone_root(&self) -> Self {
        Self {
            type_info:self.type_info,
            inner : self.inner.clone_root(),
            // caller:self.caller.clone(),
        }
    }
    pub fn clone_as_is(&self) -> Self {
        Self {
            type_info:self.type_info,
            inner : self.inner.clone_as_is(),
            // caller:self.caller.clone(),
        }
    }
    
    pub fn gc_index(&self) -> Option<usize> {
        match &self.inner {
            CustomInner::Managed(x)=>x.gc_index(),
            CustomInner::Unmanaged(_)=>None,
            // CustomInner::UnmanagedStatic(_)=>None,
            // CustomInner::None(_)=>None,
        }
    }

    pub fn is_type<T:Any>(&self) -> bool {
        self.type_info.id()==std::any::TypeId::of::<T>()
    }

    pub fn data(&self) ->CustomData {
        // self
        // self.inner
        let data=match &self.inner {
            CustomInner::Managed (x)=>{
                match &x.data {
                    WeakValueInner::Mut(x) => {
                        x.upgrade().map(|x|StrongValueInner::Mut(x))
                    }
                    WeakValueInner::NonMut(x) => {
                        x.upgrade().map(|x|StrongValueInner::NonMut(x))
                    }
                    // WeakValueInner::MutExt(x) => {
                    //     x.upgrade().map(|x|StrongValueInner::MutExt(x))
                    // }
                    // WeakValueInner::NonMutExt(x) => {
                    //     x.upgrade().map(|x|StrongValueInner::NonMutExt(x))
                    // }
                }
            },
            CustomInner::Unmanaged (data)=>Some(data.clone()),
        };

        CustomData { data, type_info : self.type_info }        
    }

    pub fn with_data_mut<T:Any,R>(&self,func:impl FnOnce(&mut T)->Result<R,MachineError>) -> Result<R,MachineError> {
        // let data=self.data();
        // let mut data=data.get_mut::<T>()?;
        // func(&mut data)
        self.data().inner_with_data_mut(func)
    }

    pub fn with_data_ref<T:Any,R>(&self,func:impl FnOnce(&T)->Result<R,MachineError>) -> Result<R,MachineError> {
        let data=self.data();
        
        match &data.data {
            Some(StrongValueInner::Mut(_))=>{
                // let data2=data.get_mut::<T>()?;
                // func(& data2)
                
                self.data().inner_with_data_mut(|data|func(data))
            }
            Some(StrongValueInner::NonMut(_))=>{
                let data2=data.get_non_mut()?;
                func(& data2)
            }
            // Some(StrongValueInner::MutExt(_))=>{
            //     let data2=data.get_mut::<T>()?;
            //     func(& data2)
            // }
            // Some(StrongValueInner::NonMutExt(_))=>{
            //     let data2=data.get_non_mut()?;
            //     func(& data2)
            // }
            None => {
                Err(MachineError::new(MachineErrorType::CustomDataEmpty))
            }
        }
    }

    pub fn data_clone<T:Any+Clone>(&self) -> Result<T,MachineError> {
        // Ok(self.data().get_mut::<T>()?.clone())
        Ok(self.with_data_ref(|x: &T|Ok(x.clone()))?)
    }
    
    pub fn data_copy<T:Any+Copy>(&self) -> Result<T,MachineError> {
        // Ok(*(self.data().get_mut::<T>()?))
        Ok(self.with_data_ref(|x: &T|Ok(*x))?)
    }

    pub fn type_info(&self) -> TypeInfo {
        self.type_info
    }

}


// pub enum CustomDataValue {
//     Dynamic(),
//     Static(),
// }
// #[derive(Clone)]
pub struct CustomData {
    data: Option<StrongValueInner>,//Option<Arc<Mutex<dyn Any+Send>>>,
    // type_name : &'static str,
    type_info : TypeInfo,
}

impl CustomData {
    pub fn get_string(&self) -> Option<String> {
        self.data.as_ref().and_then(|x|x.get_string())
    }
    pub fn get_non_mut<T:std::any::Any>(&self) -> Result<&T,MachineError> {
        if self.data.is_none() {
            return Err(MachineError::new(MachineErrorType::CustomDataEmpty));
        }

        // let x = match &self.data {
        //     StrongValueInner::NonMut(data) =>Some(data),
        //     StrongValueInner::NonMutExt(data) =>Some(),
        //     _ => None,
        // };
        let Some(StrongValueInner::NonMut(data))=&self.data else {
            return Err(MachineError::new(MachineErrorType::CustomDataNotNonMut));
        };

        let Some(data)=data.as_ref().downcast_ref() else {
            return Err(MachineError::new(MachineErrorType::CustomDataInvalidCast{
                expecting_type:TypeInfo::new::<T>().short_name(),
                given_type:self.type_info.short_name(),
            }));
        };

        Ok(data)
    }

    fn inner_with_data_mut<T:Any,R>(&self,func:impl FnOnce(&mut T)->Result<R,MachineError>) -> Result<R,MachineError> {
    
        let Some(data)=&self.data else {
            return Err(MachineError::new(MachineErrorType::CustomDataEmpty));
        };

        let StrongValueInner::Mut(data)=data else {
            return Err(MachineError::new(MachineErrorType::CustomDataNotMut));
        };

        let Ok(mut b) = data.try_lock() else {
            return Err(MachineError::new(MachineErrorType::CustomDataBorrowMutError));
        };

        let Some(b)=b.downcast_mut::<T>() else {
            return Err(MachineError::new(MachineErrorType::CustomDataInvalidCast{
                expecting_type:TypeInfo::new::<T>().short_name(),
                given_type:self.type_info.short_name(),
            }));
        };

        func(b)
    }

    // pub fn get_mut<T:std::any::Any>(&self) -> Result<MappedMutexGuard<'_, T>,MachineError> {
    //     let Some(data)=&self.data else {
    //         return Err(MachineError::new(MachineErrorType::CustomDataEmpty));
    //     };

    //     let StrongValueInner::Mut(data)=data else {
    //         return Err(MachineError::new(MachineErrorType::CustomDataNotMut));
    //     };

    //     let Ok(b) = data.try_lock() else {
    //         return Err(MachineError::new(MachineErrorType::CustomDataBorrowMutError));
    //     };
        
    //     let Ok(m)=MutexGuard::try_map(b, |x|{
    //         x.downcast_mut::<T>()
    //     }) else {
    //         return Err(MachineError::new(MachineErrorType::CustomDataInvalidCast{
    //             expecting_type:TypeInfo::new::<T>().short_name(),
    //             given_type:self.type_info.short_name(),
    //         }));
    //     };
        
    //     Ok(m)
    // }
}

