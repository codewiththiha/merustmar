use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{BlockStatement, Identifier},
    environment::Environment,
};

// Built-in function type
pub type BuiltinFunction = fn(Vec<Object>) -> Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    ReturnValue,
    ErrorObj,
    Function,
    String,
    Builtin,
    Array,
    Hash,
    Float,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "INTEGER"),
            ObjectType::Float => write!(f, "FLOAT"),
            ObjectType::Boolean => write!(f, "BOOLEAN"),
            ObjectType::Null => write!(f, "NULL"),
            ObjectType::ReturnValue => write!(f, "RETURN_VALUE"),
            ObjectType::ErrorObj => write!(f, "ERROR"),
            ObjectType::Function => write!(f, "FUNCTION"),
            ObjectType::String => write!(f, "STRING"),
            ObjectType::Builtin => write!(f, "BUILTIN"),
            ObjectType::Array => write!(f, "ARRAY"),
            ObjectType::Hash => write!(f, "HASH"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    ErrorObj(String),
    Null,
    Function(Function),
    String(String),
    Builtin(BuiltinFunction),
    Array(Vec<Object>),
    Hash(HashMap<HashKey, HashPair>),
}

// this entire thing's just necessary cuz mf BuiltinFunction causing some chaos
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::Float(a), Object::Float(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::ReturnValue(a), Object::ReturnValue(b)) => a == b,
            (Object::ErrorObj(a), Object::ErrorObj(b)) => a == b,
            (Object::Null, Object::Null) => true,
            (Object::Function(a), Object::Function(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            // Two builtins are "equal" only if they point to the same function
            (Object::Builtin(a), Object::Builtin(b)) => std::ptr::fn_addr_eq(*a, *b),
            (Object::Array(a), Object::Array(b)) => a == b,
            (Object::Hash(a), Object::Hash(b)) => a == b,
            _ => false,
        }
    }
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Float(_) => ObjectType::Float,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::ReturnValue(_) => ObjectType::ReturnValue,
            Object::ErrorObj(_) => ObjectType::ErrorObj,
            Object::Function(_) => ObjectType::Function,
            Object::String(_) => ObjectType::String,
            Object::Builtin(_) => ObjectType::Builtin,
            Object::Array(_) => ObjectType::Array,
            Object::Hash(_) => ObjectType::Hash,
        }
    }

    pub fn hash_key(&self) -> Option<HashKey> {
        match self {
            Object::Integer(i) => Some(HashKey {
                object_type: ObjectType::Integer,
                value: *i as u64,
            }),
            Object::Float(f) => Some(HashKey {
                object_type: ObjectType::Float,
                value: f.to_bits(),
            }),
            Object::Boolean(b) => Some(HashKey {
                object_type: ObjectType::Boolean,
                value: if *b { 1 } else { 0 },
            }),
            Object::String(s) => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                s.hash(&mut hasher);
                Some(HashKey {
                    object_type: ObjectType::String,
                    value: hasher.finish(),
                })
            }
            _ => None,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(i) => i.to_string(),
            Object::Float(f) => f.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Null => "null".to_string(),
            Object::ReturnValue(o) => o.to_string(),
            Object::ErrorObj(e) => e.to_string(),
            Object::Function(func) => {
                let params: Vec<String> = func.parameters.iter().map(|p| p.to_string()).collect();
                format!("fn({}) {{\n{}\n}}", params.join(", "), func.body)
            }
            Object::String(s) => s.to_string(),
            Object::Builtin(_) => "builtin function".to_string(),
            Object::Array(elements) => {
                let els: Vec<String> = elements.iter().map(|e| e.inspect()).collect();
                format!("[{}]", els.join(", "))
            }
            Object::Hash(pairs) => {
                let pair_strs: Vec<String> = pairs
                    .values()
                    .map(|pair| format!("{}: {}", pair.key.inspect(), pair.value.inspect()))
                    .collect();
                format!("{{{}}}", pair_strs.join(", "))
            }
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}

//// Hashes

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashKey {
    pub object_type: ObjectType,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashPair {
    pub key: Object,
    pub value: Object,
}

//// Version 2
// use std::{cell::RefCell, rc::Rc};
//
// use crate::{
//     ast::{BlockStatement, Identifier},
//     environment::Environment,
// };
//
// #[derive(Debug, Clone)]
// pub struct Function {
//     pub parameters: Vec<Identifier>,
//     pub body: BlockStatement,
//     pub env: Rc<RefCell<Environment>>,
// }
//
// // Manual PartialEq — skip env to avoid Rc cycle stack overflow
// impl PartialEq for Function {
//     fn eq(&self, other: &Self) -> bool {
//         self.parameters == other.parameters && self.body == other.body
//     }
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ObjectType {
//     Integer,
//     Boolean,
//     Null,
//     ReturnValue,
//     ErrorObj,
//     Function,
//     String,
//     BuiltIn,
// }
//
// pub type BuiltInFunction = fn(Vec<Object>) -> Object;
//
// impl std::fmt::Display for ObjectType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ObjectType::Integer => write!(f, "INTEGER"),
//             ObjectType::Boolean => write!(f, "BOOLEAN"),
//             ObjectType::Null => write!(f, "NULL"),
//             ObjectType::ReturnValue => write!(f, "RETURN_VALUE"),
//             ObjectType::ErrorObj => write!(f, "ERROR"),
//             ObjectType::Function => write!(f, "FUNCTION"),
//             ObjectType::String => write!(f, "STRING"),
//             ObjectType::BuiltIn => write!(f, "BUILTIN"),
//         }
//     }
// }
//
// #[derive(Debug, Clone, PartialEq)]
// pub enum Object {
//     Integer(i64),
//     Boolean(bool),
//     ReturnValue(Box<Object>),
//     ErrorObj(String),
//     Null,
//     Function(Function),
//     String(String),
//     BuiltIn(BuiltInFunction),
// }
//
// impl Object {
//     pub fn object_type(&self) -> ObjectType {
//         match self {
//             Object::Integer(_) => ObjectType::Integer,
//             Object::Boolean(_) => ObjectType::Boolean,
//             Object::Null => ObjectType::Null,
//             Object::ReturnValue(_) => ObjectType::ReturnValue,
//             Object::ErrorObj(_) => ObjectType::ErrorObj,
//             Object::Function(_) => ObjectType::Function,
//             Object::String(_) => ObjectType::String,
//             Object::BuiltIn(_) => ObjectType::BuiltIn,
//         }
//     }
//
//     pub fn inspect(&self) -> String {
//         match self {
//             Object::Integer(i) => i.to_string(),
//             Object::Boolean(b) => b.to_string(),
//             Object::Null => "null".to_string(),
//             Object::ReturnValue(o) => o.to_string(),
//             Object::ErrorObj(e) => e.to_string(),
//             Object::Function(func) => {
//                 let params: Vec<String> = func.parameters.iter().map(|p| p.to_string()).collect();
//                 format!("fn({}) {{\n{}\n}}", params.join(", "), func.body)
//             }
//             Object::String(s) => s.to_string(),
//             Object::BuiltIn(_) => "builtin function".to_string(),
//         }
//     }
// }
//
// impl std::fmt::Display for Object {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.inspect())
//     }
// }

////Version 1
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ObjectType {
//     Integer,
//     Boolean,
//     Null,
// }
//
// impl std::fmt::Display for ObjectType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ObjectType::Integer => write!(f, "INTEGER"),
//             ObjectType::Boolean => write!(f, "BOOLEAN"),
//             ObjectType::Null => write!(f, "NULL"),
//         }
//     }
// }
//
// // Object
//
// #[derive(Debug, Clone, PartialEq)]
// pub enum Object {
//     Integer(Integer),
//     Boolean(Boolean),
//     Null(Null),
// }
//
// impl Object {
//     pub fn object_type(&self) -> ObjectType {
//         match self {
//             Object::Integer(_) => ObjectType::Integer,
//             Object::Boolean(_) => ObjectType::Boolean,
//             Object::Null(_) => ObjectType::Null,
//         }
//     }
//
//     pub fn inspect(&self) -> String {
//         match self {
//             Object::Integer(i) => i.inspect(),
//             Object::Boolean(b) => b.inspect(),
//             Object::Null(n) => n.inspect(),
//         }
//     }
// }
//
// impl std::fmt::Display for Object {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.inspect())
//     }
// }
//
// // Integer
//
// #[derive(Debug, Clone, PartialEq)]
// pub struct Integer {
//     pub value: i64,
// }
//
// impl Integer {
//     pub fn new(value: i64) -> Self {
//         Integer { value }
//     }
//
//     //// &str can only be return if we are referencing the one from the outside structs in this case:
//     // format!() creates a brand new String. This String lives on the heap,
//     // but the "owner" of that string is a local variable inside the inspect function.
//     // When the inspect function finishes, that local variable is dropped,
//     // and the memory for the String is deleted.
//     // By returning &str (a reference), you are trying to give the caller a "bookmark" to a page
//     // that you just burned. Rust's borrow checker prevents this to stop your program from crashing later.
//     // that's why we can't just pass reference like we used to do
//     pub fn inspect(&self) -> String {
//         format!("{}", self.value)
//     }
// }
//
// // Boolean
//
// #[derive(Debug, Clone, PartialEq)]
// pub struct Boolean {
//     pub value: bool,
// }
//
// impl Boolean {
//     pub fn new(value: bool) -> Self {
//         Boolean { value }
//     }
//
//     pub fn inspect(&self) -> String {
//         format!("{}", self.value)
//     }
// }
//
// // Null
//
// #[derive(Debug, Clone, PartialEq)]
// pub struct Null;
//
// impl Null {
//     pub fn new() -> Self {
//         Null
//     }
//
//     pub fn inspect(&self) -> String {
//         "null".to_string()
//     }
// }
