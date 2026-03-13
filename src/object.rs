#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "INTEGER"),
            ObjectType::Boolean => write!(f, "BOOLEAN"),
            ObjectType::Null => write!(f, "NULL"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Null => "null".to_string(),
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}

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
