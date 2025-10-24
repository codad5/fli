
#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum ValueTypes {
    RequiredSingle(Value),
    OptionalSingle(Option<Value>),
    RequiredMultiple(Vec<Value>, Option<usize>),
    OptionalMultiple(Option<Vec<Value>>, Option<usize>),
    None,
}



impl ValueTypes {
    pub fn expects_value(&self) -> bool {
        match self {
            ValueTypes::RequiredSingle(_) => true,
            ValueTypes::OptionalSingle(_) => true,
            ValueTypes::RequiredMultiple(_, _) => true,
            ValueTypes::OptionalMultiple(_, _) => true,
            ValueTypes::None => false,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            ValueTypes::RequiredSingle(Value::Str(s)) => Some(s),
            ValueTypes::OptionalSingle(Some(Value::Str(s))) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_strings(&self) -> Option<Vec<&str>> {
        match self {
            ValueTypes::RequiredMultiple(values, _) | 
            ValueTypes::OptionalMultiple(Some(values), _) => {
                Some(values.iter().filter_map(|v| {
                    if let Value::Str(s) = v { Some(s.as_str()) } else { None }
                }).collect())
            }
            _ => None,
        }
    }
}