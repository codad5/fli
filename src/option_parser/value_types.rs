use crate::error::{FliError, Result};
/// Represents a typed value parsed from command-line arguments.
///
/// Supports common primitive types used in CLI applications.
///
/// # Examples
///
/// ```rust
/// let str_val = Value::Str("hello".to_string());
/// let int_val = Value::Int(42);
/// let float_val = Value::Float(3.14);
/// let bool_val = Value::Bool(true);
/// ```
#[derive(Debug, Clone)]
pub enum Value {
    /// A string value
    Str(String),

    /// An integer value (64-bit signed)
    Int(i64),

    /// A floating-point value (64-bit)
    Float(f64),

    /// A boolean value
    Bool(bool),
}

impl Value {
    /// Replaces the current value with a new value parsed from a string.
    ///
    /// This method attempts to parse the input string according to the expected type
    /// and updates the value in place if successful.
    ///
    /// # Arguments
    ///
    /// * `new_value` - The string representation of the new value
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - A clone of the updated value
    /// * `Err(FliError)` - If parsing fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut val = Value::Int(0);
    /// val.replace_with_expected_value("42").unwrap();
    /// assert_eq!(val, Value::Int(42));
    ///
    /// let mut val = Value::Bool(false);
    /// val.replace_with_expected_value("yes").unwrap();
    /// assert_eq!(val, Value::Bool(true));
    /// ```
    pub fn replace_with_expected_value(&mut self, new_value: &str) -> Result<Value> {
        match self {
            Value::Str(s) => {
                *s = new_value.to_string();
                Ok(self.clone())
            }
            Value::Int(i) => {
                match new_value.parse::<i64>() {
                    Ok(v) => {
                        *i = v;
                        Ok(self.clone())
                    }
                    Err(e) => Err(FliError::ValueParseError {
                        value: new_value.to_string(),
                        expected_type: "integer (i64)".to_string(),
                        reason: e.to_string(),
                    }),
                }
            }
            Value::Float(f) => {
                match new_value.parse::<f64>() {
                    Ok(v) => {
                        *f = v;
                        Ok(self.clone())
                    }
                    Err(e) => Err(FliError::ValueParseError {
                        value: new_value.to_string(),
                        expected_type: "float (f64)".to_string(),
                        reason: e.to_string(),
                    }),
                }
            }
            Value::Bool(b) => {
                let v = match new_value.to_ascii_lowercase().as_str() {
                    "true" | "t" | "1" | "yes" | "y" => Some(true),
                    "false" | "f" | "0" | "no" | "n" => Some(false),
                    _ => None,
                };
                match v {
                    Some(val) => {
                        *b = val;
                        Ok(self.clone())
                    }
                    None => Err(FliError::ValueParseError {
                        value: new_value.to_string(),
                        expected_type: "boolean".to_string(),
                        reason: "expected one of: true, false, t, f, 1, 0, yes, no, y, n (case-insensitive)".to_string(),
                    }),
                }
            }
        }
    }

    /// Creates a new `Value` from a string, attempting to parse it according to the type.
    ///
    /// This is similar to `replace_with_expected_value` but creates a new value instead
    /// of modifying an existing one.
    ///
    /// # Arguments
    ///
    /// * `template` - A template value indicating the expected type
    /// * `input` - The string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The parsed value
    /// * `Err(FliError)` - If parsing fails
    pub fn from_str_with_type(template: &Value, input: &str) -> Result<Value> {
        let mut temp = template.clone();
        temp.replace_with_expected_value(input)?;
        Ok(temp)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => false,
        }
    }
}

/// Defines the type and cardinality of values an option can accept.
///
/// This enum enforces compile-time guarantees about option value requirements:
/// - Whether values are required or optional
/// - Whether single or multiple values are accepted
/// - Default values when not provided
///
/// # Examples
///
/// ```rust
/// // Flag option (no value)
/// let verbose = ValueTypes::None;
///
/// // Required single value
/// let output = ValueTypes::RequiredSingle(Value::Str(String::new()));
///
/// // Optional single value with default
/// let port = ValueTypes::OptionalSingle(Some(Value::Int(8080)));
///
/// // Multiple required values (at least 1)
/// let files = ValueTypes::RequiredMultiple(vec![], None);
///
/// // Exactly 3 values required
/// let coords = ValueTypes::RequiredMultiple(vec![], Some(3));
///
/// // Multiple optional values
/// let tags = ValueTypes::OptionalMultiple(None, None);
/// ```
#[derive(Debug, Clone)]
pub enum ValueTypes {
    /// Requires exactly one value
    /// - First field: default value
    RequiredSingle(Value),

    /// Optionally accepts one value
    /// - First field: default value if not provided
    OptionalSingle(Option<Value>),

    /// Requires at least one value, optionally up to a maximum
    /// - First field: collected values
    /// - Second field: exact count required (None = unlimited)
    RequiredMultiple(Vec<Value>, Option<usize>),

    /// Optionally accepts multiple values
    /// - First field: collected values if any
    /// - Second field: maximum count (None = unlimited)
    OptionalMultiple(Option<Vec<Value>>, Option<usize>),

    /// Flag option that doesn't accept values
    None,
}

impl ValueTypes {
    /// Checks if this value type expects to consume arguments.
    ///
    /// # Returns
    ///
    /// `true` for all types except `None`
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(ValueTypes::None.expects_value(), false);
    /// assert_eq!(ValueTypes::RequiredSingle(Value::Str(s)).expects_value(), true);
    /// ```
    pub fn expects_value(&self) -> bool {
        match self {
            ValueTypes::RequiredSingle(_) => true,
            ValueTypes::OptionalSingle(_) => true,
            ValueTypes::RequiredMultiple(_, _) => true,
            ValueTypes::OptionalMultiple(_, _) => true,
            ValueTypes::None => false,
        }
    }

    /// Extracts a string value if this is a single-value type.
    ///
    /// # Returns
    ///
    /// * `Some(&str)` - If this contains a string value
    /// * `None` - If this is not a single string value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let val = ValueTypes::RequiredSingle(Value::Str("hello".to_string()));
    /// assert_eq!(val.as_str(), Some("hello"));
    ///
    /// let val = ValueTypes::None;
    /// assert_eq!(val.as_str(), None);
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            ValueTypes::RequiredSingle(Value::Str(s)) => Some(s),
            ValueTypes::OptionalSingle(Some(Value::Str(s))) => Some(s),
            _ => None,
        }
    }

    /// Extracts multiple string values if this is a multi-value type.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<&str>)` - Vector of string slices if values are strings
    /// * `None` - If this is not a multi-value type or contains non-strings
    ///
    /// # Examples
    ///
    /// ```rust
    /// let values = vec![
    ///     Value::Str("file1.txt".to_string()),
    ///     Value::Str("file2.txt".to_string()),
    /// ];
    /// let val = ValueTypes::RequiredMultiple(values, None);
    ///
    /// let strings = val.as_strings().unwrap();
    /// assert_eq!(strings, vec!["file1.txt", "file2.txt"]);
    /// ```
    pub fn as_strings(&self) -> Option<Vec<&str>> {
        match self {
            ValueTypes::RequiredMultiple(values, _)
            | ValueTypes::OptionalMultiple(Some(values), _) => Some(
                values
                    .iter()
                    .filter_map(|v| {
                        if let Value::Str(s) = v {
                            Some(s.as_str())
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
            _ => None,
        }
    }
}
