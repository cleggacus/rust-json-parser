use std::{ops::{Index}, fmt::{Display, Debug, self}};

#[derive(Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<Node>)
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => f.write_str("Null"),
            Value::Boolean(val) => write!(f, "{}", val),
            Value::Number(val) => write!(f, "{}", val),
            Value::String(val) => write!(f, "\"{}\"", val),
            Value::Array(vals) => write!(f, "{:#?}", vals),
            Value::Object(nodes) => {
                let string = format!("{:#?}", nodes);
                let string = string.split_at(string.len() - 1);
                let string = string.0.split_at(1);
                let string = string.1;

                write!(f, "{{{}}}", string)
            }
        //     {
        //         write!(f, "{{\n")?;

        //         for i in 0..nodes.len() {
        //             write!(f, "{:#?}", nodes[i])?;

        //             if i < nodes.len() - 1 {
        //                 write!(f, ", \n")?;
        //             }
        //         }

        //         write!(f, "\n}}")
        //     }
        }
    }
}


impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Index<&str> for Value {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Value::Object(nodes) => {
                for node in nodes {
                    if node.key == index {
                        return &node.value;
                    }
                }

                return &Value::Null;
            }
            _ => &Value::Null
        }
    }
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Array(vals) => {
                if index < vals.len() {
                    return &vals[index];
                }

                return &Value::Null;
            }
            _ => &Value::Null
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub key: String,
    pub value: Value
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.key, self.value)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
