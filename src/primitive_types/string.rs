use std::{hash::Hash, sync::Arc};


#[derive(Clone,Debug,Eq,Ord)]
pub enum StringVal {
    Str(&'static str),
    String(Arc<String>),
}

impl StringVal {
    pub fn new<S: Into<String>>(x:S) -> Self {
        Self::String(Arc::new(x.into()))
    }

    pub fn new_str(value: &'static str) -> Self {
        Self::Str(value)
    }

    pub fn as_str(&self) -> &str {
        match self {
            StringVal::Str(s) => s,
            StringVal::String(s) => s.as_str(),
        }
    }
}

impl Into<String> for StringVal {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl Into<Arc<String>> for StringVal {
    fn into(self) -> Arc<String> {
        match self {
            StringVal::Str(s) => Arc::new(s.to_string()),
            StringVal::String(s) => s.clone(),
        }
    }
}

impl From<String> for StringVal {
    fn from(value: String) -> Self {
        Self::String(Arc::new(value))
    }
}

// impl<'a> From<&'a str> for StringVal {
//     fn from(value: &'a str) -> Self {
//         Self::String(Arc::new(value.to_string()))
//     }
// }

impl From<&'static str> for StringVal {
    fn from(value: &'static str) -> Self {
        Self::Str(value)
    }
}

impl Hash for StringVal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            StringVal::Str(a) =>a.hash(state),
            StringVal::String(a) => a.hash(state),
        }
    }
}

impl PartialEq for StringVal {
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl PartialOrd for StringVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_str().cmp(&other.as_str()))
    }
}
//Hash,PartialEq, Eq,PartialOrd, Ord