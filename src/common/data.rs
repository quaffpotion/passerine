use std::{
    fmt::{
        Debug,
        Display,
        Formatter,
        Result,
    },
    f64,
    rc::Rc,
    cell::RefCell,
};

use crate::common::{
    lambda::Lambda,
    closure::Closure,
};

/// Built-in Passerine datatypes.
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    // VM Stack
    Frame,
    Heaped(Rc<RefCell<Data>>),

    // Passerine Data (Atomic)
    Real(f64),
    Boolean(bool),
    String(String),
    Lambda(Lambda),
    Closure(Closure),
    Label(String, Box<Data>), // TODO: better type

    // Compound Datatypes
    Unit, // an empty typle
    // Tuple(Vec<Data>),
    // // TODO: Hashmap?
    // // I mean, it's overkill for small things
    // // yet if people have very big records, yk.
    // Record(Vec<(Local, Data)>),
    // ArbInt(ArbInt),
}

// TODO: manually implement the equality trait
// NOTE: might have to implement partial equality as well
// NOTE: equality represents passerine equality, not rust equality
impl Eq for Data {}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Data::Frame       => unreachable!("Can not display stack frame"),
            Data::Heaped(_)   => unreachable!("Can not display heaped data"),
            Data::Real(n)     => write!(f, "Real {}", n),
            Data::Boolean(b)  => write!(f, "Boolean {}", if *b { "true" } else { "false" }),
            Data::String(s)   => write!(f, "{}", s),
            Data::Lambda(_)   => write!(f, "Function"),
            Data::Closure(_)  => write!(f, "Closure"),
            Data::Label(n, v) => write!(f, "{} {}", n, v),
            Data::Unit        => write!(f, "()"),
        }
    }
}
