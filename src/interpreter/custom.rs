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
use std::sync::Weak;
// use std::sync::MutexGuard;


use crate::interpreter::gc::GcValue;
use crate::interpreter::gc::TypeInfo;
use crate::GcScope;
use crate::GcTraversable;

// use super::gc_scope::*;
// use super::value::*;
// use super::error::*;


#[derive(Clone)]
pub enum StrongValueInner {
    Mut(Arc<Mutex<dyn Any+Send>>),
    NonMut(Arc<dyn Any+Send+Sync>),
    // Dead, //for manageds, when using get (strong) data, also used for rc_weak => rc_strong
    // Empty, //used for when getting data from something that isn't a custom, instead of failing, return empty data ?

}

impl StrongValueInner {
    pub fn is_mut(&self) -> bool {
        match self {
            StrongValueInner::Mut(_) => true,
            StrongValueInner::NonMut(_) => false,
        }
    }

    pub fn downgrade(&self) -> WeakValueInner {
        match self {
            Self::Mut(x) => WeakValueInner::Mut(Arc::downgrade(x)),
            Self::NonMut(x) => WeakValueInner::NonMut(Arc::downgrade(x)),
        }
    }
}

#[derive(Clone)]
pub enum WeakValueInner {
    Mut(Weak<Mutex<dyn Any+Send>>),
    NonMut(Weak<dyn Any+Send+Sync>),
}

impl WeakValueInner {
    pub fn is_mut(&self) -> bool {
        match self {
            WeakValueInner::Mut(_) => true,
            WeakValueInner::NonMut(_) => false,
        }
    }
    pub fn is_alive(&self) -> bool {
        match self {
            WeakValueInner::Mut(weak) => weak.strong_count()!=0,
            WeakValueInner::NonMut(weak) => weak.strong_count()!=0,
        }
    }
    pub fn upgrade(&self) -> Option<StrongValueInner> {
        match self {
            Self::Mut(x)=>x.upgrade().map(|x|StrongValueInner::Mut(x)),
            Self::NonMut(x)=>x.upgrade().map(|x|StrongValueInner::NonMut(x)),
        }
    }
}

#[derive(Clone)]
pub enum CustomInner {
    Managed(GcValue),
    Unmanaged(StrongValueInner),
    UnmanagedWeak(WeakValueInner),
    Empty,
}

impl CustomInner {
    pub fn to_strong(&self) -> Option<Self> {
        match self {
            Self::Managed(_) => None,
            Self::Unmanaged(_) => None,
            Self::UnmanagedWeak(w) => w.upgrade().map(|s|Self::Unmanaged(s)),
            Self::Empty => None,
        }
    }

    pub fn to_weak(&self) -> Option<Self> {
        match self {
            Self::Managed(_) => None,
            Self::Unmanaged(s) => Some(Self::UnmanagedWeak(s.downgrade())),
            Self::UnmanagedWeak(_) => None,
            Self::Empty => None,
        }
    }

    pub fn clone_root(&self) -> Self {
        match self {
            Self::Managed(x)=>Self::Managed(x.clone_root()),
            Self::Unmanaged(x)=>Self::Unmanaged(x.clone()),
            Self::UnmanagedWeak(x) => Self::UnmanagedWeak(x.clone()),
            Self::Empty => Self::Empty,
        }
    }
    pub fn clone_as_is(&self) -> Self {
        match self {
            Self::Managed(x)=>Self::Managed(x.clone_as_is()),
            Self::Unmanaged(x)=>Self::Unmanaged(x.clone()),
            Self::UnmanagedWeak(x) => Self::UnmanagedWeak(x.clone()),
            Self::Empty => Self::Empty,
        }
    }
    pub fn clone_leaf(&self) -> Self {
        match self {
            Self::Managed(x)=>Self::Managed(x.clone_leaf()),
            Self::Unmanaged(x)=>Self::Unmanaged(x.clone()),
            Self::UnmanagedWeak(x) => Self::UnmanagedWeak(x.clone()),
            Self::Empty => Self::Empty,
        }
    }
}

#[derive(Clone)]

pub struct Custom {
    type_info:TypeInfo,
    inner : CustomInner,
}


impl std::string::ToString for Custom {
    fn to_string(&self) -> String {
        self.type_info.short_name()
    }
}

impl std::fmt::Debug for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.inner {
            CustomInner::Managed(_) => {
                match self.gc_index() {
                    Ok(Some(gc_index)) => {
                        write!(f, "Managed:{gc_index}:({})",self.type_info.short_name())
                    }
                    Ok(None)=> {
                        write!(f, "Managed:Dead:({})",self.type_info.short_name())
                    }
                    Err(_) => {
                        write!(f, "Managed:Locked:({})",self.type_info.short_name())
                    }
                }

            }
            CustomInner::Unmanaged(_) => {
                write!(f, "Unmanaged:({})",self.type_info.short_name())
            }
            CustomInner::UnmanagedWeak(_) => {
                write!(f, "UnmanagedWeak:({})",self.type_info.short_name())
            }
            CustomInner::Empty => {
                write!(f, "Empty")
            }
        }
    }
}

impl Custom {
    pub fn to_strong(&self) -> Option<Self> {
        self.inner.to_strong().map(|inner|Self { type_info: self.type_info, inner })
    }

    pub fn to_weak(&self) -> Option<Self> {
        self.inner.to_weak().map(|inner|Self { type_info: self.type_info, inner })
    }

    pub fn is_unmanaged_strong(&self) -> bool {
        match &self.inner {
            CustomInner::Managed(_) => false,
            CustomInner::Unmanaged(_) => true,
            CustomInner::UnmanagedWeak(_) => false,
            CustomInner::Empty => false,
        }
    }
    pub fn is_unmanaged_weak(&self) -> bool {
        match &self.inner {
            CustomInner::Managed(_) => false,
            CustomInner::Unmanaged(_) => false,
            CustomInner::UnmanagedWeak(_) => true,
            CustomInner::Empty => false,
        }
    }

    pub fn is_mut(&self) -> bool {
        match &self.inner {
            CustomInner::Managed(gc_value) => gc_value.data.is_mut(),
            CustomInner::Unmanaged(strong_value_inner) => strong_value_inner.is_mut(),
            CustomInner::UnmanagedWeak(weak_value_inner) => weak_value_inner.is_mut(),
            CustomInner::Empty => false,
        }
    }
    pub fn is_not_mut(&self) -> bool {
        !self.is_mut()
    }

    pub fn is_managed(&self) -> bool {
        if let CustomInner::Managed(_)=self.inner { true } else { false }
    }
    pub fn is_unmanaged(&self) -> bool {
        match &self.inner {
            CustomInner::Unmanaged(_) => true,
            CustomInner::UnmanagedWeak(_) => true,
            _ => false,
        }
    }
    pub fn is_empty(&self) -> bool {
        if let CustomInner::Empty=self.inner { true } else { false }
    }

    pub fn new<T:Any>(inner:CustomInner) -> Self {
        Self {
            type_info:TypeInfo::new::<T>(),
            inner,
        }
    }


    // pub fn new_managed_mut2<T:GcTraversable+Send>(data : T,mut gc_new : impl FnMut(T)->GcValue) -> Self {
    //     Self::new::<T>(CustomInner::Managed(gc_new(data)))
    // }

    // pub fn new_managed2<T:GcTraversable+Send+Sync>(data : T,mut gc_new : impl FnMut(T)->GcValue) -> Self {
    //     Self::new::<T>(CustomInner::Managed(GcValue::new_non_mut(data,gc_scope)))
    // }

    pub fn new_managed_mut<T:GcTraversable+Send>(data : T,gc_scope : &mut GcScope) -> Self {
        // Self::new::<T>(CustomInner::Managed(GcValue::new_mut(data,gc_scope)))
        // Self::new::<T>(CustomInner::Managed(GcValue::new(gc_scope.new_mut(data))))
        Self::new::<T>(CustomInner::Managed(gc_scope.new_mut(data)))
    }

    pub fn new_managed<T:GcTraversable+Send+Sync>(data : T, gc_scope : &mut GcScope) -> Self {
        // Self::new::<T>(CustomInner::Managed(GcValue::new_non_mut(data,gc_scope)))
        // Self::new::<T>(CustomInner::Managed(GcValue::new(gc_scope.new_non_mut(data))))
        Self::new::<T>(CustomInner::Managed(gc_scope.new_non_mut(data)))
    }

    // pub fn new_managed()

    pub fn new_unmanaged_mut<T:Any+Send>(data : T) -> Self {
        Self::new::<T>(CustomInner::Unmanaged(StrongValueInner::Mut(Arc::new(Mutex::new(data)))))
    }

    pub fn new_unmanaged<T:Any+Send+Sync>(data : T) -> Self {
        Self::new::<T>(CustomInner::Unmanaged(StrongValueInner::NonMut(Arc::new(data))))
    }

    pub fn new_empty() -> Self {
        Self { type_info: TypeInfo::new::<()>(), inner: CustomInner::Empty }
    }

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
    pub fn clone_leaf(&self) -> Self {
        Self {
            type_info:self.type_info,
            inner : self.inner.clone_leaf(),
        }
    }

    pub fn gc_index(&self) -> Result<Option<usize>,()> {
        match &self.inner {
            CustomInner::Managed(x)=>x.gc_index(),
            CustomInner::Unmanaged(_)=>Ok(None),
            CustomInner::UnmanagedWeak(_)=>Ok(None),
            CustomInner::Empty => Ok(None),
        }
    }

    pub fn is_type<T:Any>(&self) -> bool {
        self.type_info.id()==std::any::TypeId::of::<T>()
    }

    pub fn data(&self) ->CustomData {
        let data: Option<StrongValueInner>=match &self.inner {
            CustomInner::Managed (x)=>{
                match &x.data {
                    WeakValueInner::Mut(x) => {
                        x.upgrade().map(|x|StrongValueInner::Mut(x)) //.unwrap_or(StrongValueInner::Dead)
                    }
                    WeakValueInner::NonMut(x) => {
                        x.upgrade().map(|x|StrongValueInner::NonMut(x)) //.unwrap_or(StrongValueInner::Dead)
                    }
                }
            },
            CustomInner::Unmanaged (data)=>Some(data.clone()),
            CustomInner::UnmanagedWeak (x)=>x.upgrade(),
            CustomInner::Empty => None, //StrongValueInner::Dead,
        };

        CustomData { data, type_info : self.type_info }
    }

    pub fn with_data_mut<T:Any>(&self,func:impl FnOnce(&mut T)) -> Result<(),CustomError> {
        self.with_data_mut_ext(|data|{
            func(data);
            Ok(())
        })
    }
    pub fn with_data_ref<T:Any>(&self,func:impl FnOnce(&T)) -> Result<(),CustomError> {
        self.with_data_ref_ext(|data|{
            func(data);
            Ok(())
        })
    }

    // pub fn with_data_mut_ret<T:Any,R>(&self,func:impl FnOnce(&mut T)->R) -> Result<R,CustomError> {
    //     self.with_data_mut_ext(|data|{
    //         Ok(func(data))
    //     })
    // }
    pub fn with_data_ref_ret<T:Any,R>(&self,func:impl FnOnce(&T)->R) -> Result<R,CustomError> {
        self.with_data_ref_ext(|data|{
            Ok(func(data))
        })
    }

    pub fn with_data_mut_ext<T:Any,R,E:From<CustomError>>(&self,func:impl FnOnce(&mut T)->Result<R,E>) -> Result<R,E> {
        self.data().inner_with_data_mut(func)
    }

    pub fn with_data_ref_ext<T:Any,R,E:From<CustomError>>(&self,func:impl FnOnce(&T)->Result<R,E>) -> Result<R,E> {
        let data=self.data();

        match &data.data {
            Some(StrongValueInner::Mut(_))=>data.inner_with_data_mut(|data|func(data)),
            Some(StrongValueInner::NonMut(_))=>func(data.get_non_mut::<T>()?),
            None => Err(CustomError::CustomDataDead.into()),
        }
    }
    pub fn get_data_clone<T:Any+Clone>(&self) -> Result<Option<T>,CustomError> {
        match self.with_data_ref_ext(|x: &T|Ok(x.clone())) {
            Ok(x) => Ok(Some(x)),
            Err(CustomError::CustomDataInvalidCast{..}) => Ok(None),
            Err(e) => Err(e)
        }
    }

    pub fn data_clone<T:Any+Clone>(&self) -> Result<T,CustomError> {
        // Ok(self.data().get_mut::<T>()?.clone())
        Ok(self.with_data_ref_ext(|x: &T|Ok(x.clone()))?)
    }

    pub fn type_info(&self) -> TypeInfo {
        self.type_info
    }

    pub fn is_alive(&self) -> bool{
        match &self.inner {
            CustomInner::Managed(gc_value) => gc_value.is_alive(),
            CustomInner::Unmanaged(_) => true,
            CustomInner::UnmanagedWeak(w) => w.is_alive(),
            CustomInner::Empty => false,
        }
    }
}


// #[derive(Clone)]
pub struct CustomData {
    // data: StrongValueInner, //Option<>,//Option<Arc<Mutex<dyn Any+Send>>>,
    data: Option<StrongValueInner>,
    // type_name : &'static str,
    type_info : TypeInfo,
}

impl CustomData {
    pub fn get_non_mut<T:std::any::Any>(&self) -> Result<&T,CustomError> {
        match &self.data {
            Some(StrongValueInner::Mut(_)) => Err(CustomError::CustomDataNotNonMut),
            Some(StrongValueInner::NonMut(data2)) => {
                if let Some(data)=data2.as_ref().downcast_ref() {
                    Ok(data)
                } else {
                    Err(CustomError::CustomDataInvalidCast{
                        expecting_type:TypeInfo::new::<T>().short_name(),
                        given_type:self.type_info.short_name(),
                    })
                }
            },
            None => Err(CustomError::CustomDataDead),
        }

    }

    fn inner_with_data_mut<T:Any,R,E:From<CustomError>>(&self,func:impl FnOnce(&mut T)->Result<R,E>) -> Result<R,E> {
        match &self.data {
            Some(StrongValueInner::Mut(data2)) => {
                if let Ok(mut b) = data2.try_lock() {
                    if let Some(b)=b.downcast_mut::<T>() {
                        func(b)
                    } else {
                        return Err(CustomError::CustomDataInvalidCast{
                            expecting_type:TypeInfo::new::<T>().short_name(),
                            given_type:self.type_info.short_name(),
                        }.into());
                    }
                } else {
                    return Err(CustomError::CustomDataBorrowMutError.into());
                }
            },
            Some(StrongValueInner::NonMut(_)) => Err(CustomError::CustomDataNotMut.into()),
            None => Err(CustomError::CustomDataDead.into()),
        }
    }

}

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
