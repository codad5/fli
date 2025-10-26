use crate::option_parser::{Value, ValueTypes};

#[test]
fn test_value_str_creation() {
    let value = Value::Str("hello".to_string());
    match value {
        Value::Str(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected Str variant"),
    }
}

#[test]
fn test_value_int_creation() {
    let value = Value::Int(42);
    match value {
        Value::Int(n) => assert_eq!(n, 42),
        _ => panic!("Expected Int variant"),
    }
}

#[test]
fn test_value_float_creation() {
    let value = Value::Float(3.14);
    match value {
        Value::Float(f) => assert!((f - 3.14).abs() < 0.001),
        _ => panic!("Expected Float variant"),
    }
}

#[test]
fn test_value_bool_creation() {
    let value = Value::Bool(true);
    match value {
        Value::Bool(b) => assert!(b),
        _ => panic!("Expected Bool variant"),
    }
}

#[test]
fn test_value_types_none_expects_value() {
    let vt = ValueTypes::None;
    assert!(!vt.expects_value());
}

#[test]
fn test_value_types_required_single_expects_value() {
    let vt = ValueTypes::RequiredSingle(Value::Str("default".to_string()));
    assert!(vt.expects_value());
}

#[test]
fn test_value_types_optional_single_expects_value() {
    let vt = ValueTypes::OptionalSingle(Some(Value::Int(0)));
    assert!(vt.expects_value());
}

#[test]
fn test_value_types_required_multiple_expects_value() {
    let vt = ValueTypes::RequiredMultiple(vec![], None);
    assert!(vt.expects_value());
}

#[test]
fn test_value_types_optional_multiple_expects_value() {
    let vt = ValueTypes::OptionalMultiple(None, None);
    assert!(vt.expects_value());
}

#[test]
fn test_as_str_with_required_single() {
    let vt = ValueTypes::RequiredSingle(Value::Str("test".to_string()));
    assert_eq!(vt.as_str(), Some("test"));
}

#[test]
fn test_as_str_with_optional_single() {
    let vt = ValueTypes::OptionalSingle(Some(Value::Str("optional".to_string())));
    assert_eq!(vt.as_str(), Some("optional"));
}

#[test]
fn test_as_str_with_none() {
    let vt = ValueTypes::None;
    assert_eq!(vt.as_str(), None);
}

#[test]
fn test_as_str_with_int() {
    let vt = ValueTypes::RequiredSingle(Value::Int(42));
    assert_eq!(vt.as_str(), None);
}

#[test]
fn test_as_strings_with_required_multiple() {
    let values = vec![
        Value::Str("file1.txt".to_string()),
        Value::Str("file2.txt".to_string()),
        Value::Str("file3.txt".to_string()),
    ];
    let vt = ValueTypes::RequiredMultiple(values, None);
    
    let strings = vt.as_strings().unwrap();
    assert_eq!(strings.len(), 3);
    assert_eq!(strings[0], "file1.txt");
    assert_eq!(strings[1], "file2.txt");
    assert_eq!(strings[2], "file3.txt");
}

#[test]
fn test_as_strings_with_optional_multiple() {
    let values = vec![
        Value::Str("arg1".to_string()),
        Value::Str("arg2".to_string()),
    ];
    let vt = ValueTypes::OptionalMultiple(Some(values), None);
    
    let strings = vt.as_strings().unwrap();
    assert_eq!(strings.len(), 2);
    assert_eq!(strings[0], "arg1");
    assert_eq!(strings[1], "arg2");
}

#[test]
fn test_as_strings_with_none() {
    let vt = ValueTypes::None;
    assert_eq!(vt.as_strings(), None);
}

#[test]
fn test_as_strings_with_single() {
    let vt = ValueTypes::RequiredSingle(Value::Str("single".to_string()));
    assert_eq!(vt.as_strings(), None);
}

#[test]
fn test_as_strings_filters_non_strings() {
    let values = vec![
        Value::Str("string1".to_string()),
        Value::Int(42),
        Value::Str("string2".to_string()),
        Value::Bool(true),
    ];
    let vt = ValueTypes::RequiredMultiple(values, None);
    
    let strings = vt.as_strings().unwrap();
    assert_eq!(strings.len(), 2);
    assert_eq!(strings[0], "string1");
    assert_eq!(strings[1], "string2");
}

#[test]
fn test_value_clone() {
    let value1 = Value::Str("test".to_string());
    let value2 = value1.clone();
    
    match (value1, value2) {
        (Value::Str(s1), Value::Str(s2)) => assert_eq!(s1, s2),
        _ => panic!("Clone failed"),
    }
}

#[test]
fn test_value_types_clone() {
    let vt1 = ValueTypes::RequiredSingle(Value::Int(100));
    let vt2 = vt1.clone();
    
    assert!(vt2.expects_value());
}
