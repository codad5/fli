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
    let vt = ValueTypes::OptionalSingle(Some(Value::Bool(false)));
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
    let vt = ValueTypes::OptionalSingle(Some(Value::Bool(false)));
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
    let vt = ValueTypes::OptionalSingle(Some(Value::Bool(false)));
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

// Tests for Value::replace_with_expected_value

#[test]
fn test_replace_string_value() {
    let mut val = Value::Str("old".to_string());
    let result = val.replace_with_expected_value("new");

    assert!(result.is_ok());
    assert_eq!(val, Value::Str("new".to_string()));
}

#[test]
fn test_replace_int_value_success() {
    let mut val = Value::Int(0);
    let result = val.replace_with_expected_value("42");

    assert!(result.is_ok());
    assert_eq!(val, Value::Int(42));
}

#[test]
fn test_replace_int_value_negative() {
    let mut val = Value::Int(0);
    let result = val.replace_with_expected_value("-123");

    assert!(result.is_ok());
    assert_eq!(val, Value::Int(-123));
}

#[test]
fn test_replace_int_value_large() {
    let mut val = Value::Int(0);
    let result = val.replace_with_expected_value("9223372036854775807"); // i64::MAX

    assert!(result.is_ok());
    assert_eq!(val, Value::Int(9223372036854775807));
}

#[test]
fn test_replace_int_value_failure() {
    let mut val = Value::Int(0);
    let result = val.replace_with_expected_value("not_a_number");

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("integer"));
}

#[test]
fn test_replace_int_value_float_string() {
    let mut val = Value::Int(0);
    let result = val.replace_with_expected_value("3.14");

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("integer"));
}

#[test]
fn test_replace_float_value_success() {
    let mut val = Value::Float(0.0);
    let result = val.replace_with_expected_value("3.14");

    assert!(result.is_ok());
    assert_eq!(val, Value::Float(3.14));
}

#[test]
fn test_replace_float_value_negative() {
    let mut val = Value::Float(0.0);
    let result = val.replace_with_expected_value("-2.5");

    assert!(result.is_ok());
    assert_eq!(val, Value::Float(-2.5));
}

#[test]
fn test_replace_float_value_scientific() {
    let mut val = Value::Float(0.0);
    let result = val.replace_with_expected_value("1.5e10");

    assert!(result.is_ok());
    assert_eq!(val, Value::Float(1.5e10));
}

#[test]
fn test_replace_float_value_integer_string() {
    let mut val = Value::Float(0.0);
    let result = val.replace_with_expected_value("42");

    assert!(result.is_ok());
    assert_eq!(val, Value::Float(42.0));
}

#[test]
fn test_replace_float_value_failure() {
    let mut val = Value::Float(0.0);
    let result = val.replace_with_expected_value("not_a_float");

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("float"));
}

#[test]
fn test_replace_bool_value_true_variants() {
    let test_cases = vec![
        "true", "True", "TRUE", "t", "T", "1", "yes", "YES", "y", "Y",
    ];

    for input in test_cases {
        let mut val = Value::Bool(false);
        let result = val.replace_with_expected_value(input);

        assert!(result.is_ok(), "Failed for input: {}", input);
        assert_eq!(val, Value::Bool(true), "Failed for input: {}", input);
    }
}

#[test]
fn test_replace_bool_value_false_variants() {
    let test_cases = vec![
        "false", "False", "FALSE", "f", "F", "0", "no", "NO", "n", "N",
    ];

    for input in test_cases {
        let mut val = Value::Bool(true);
        let result = val.replace_with_expected_value(input);

        assert!(result.is_ok(), "Failed for input: {}", input);
        assert_eq!(val, Value::Bool(false), "Failed for input: {}", input);
    }
}

#[test]
fn test_replace_bool_value_failure() {
    let mut val = Value::Bool(false);
    let result = val.replace_with_expected_value("maybe");

    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(err_msg.contains("boolean"));
    assert!(err_msg.contains("true") || err_msg.contains("false"));
}

#[test]
fn test_replace_bool_value_invalid_number() {
    let mut val = Value::Bool(false);
    let result = val.replace_with_expected_value("2");

    assert!(result.is_err());
}

#[test]
fn test_from_str_with_type_string() {
    let template = Value::Str(String::new());
    let result = Value::from_str_with_type(&template, "hello");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Str("hello".to_string()));
}

#[test]
fn test_from_str_with_type_int() {
    let template = Value::Int(0);
    let result = Value::from_str_with_type(&template, "999");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Int(999));
}

#[test]
fn test_from_str_with_type_float() {
    let template = Value::Float(0.0);
    let result = Value::from_str_with_type(&template, "2.718");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Float(2.718));
}

#[test]
fn test_from_str_with_type_bool() {
    let template = Value::Bool(false);
    let result = Value::from_str_with_type(&template, "yes");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Bool(true));
}

#[test]
fn test_from_str_with_type_int_failure() {
    let template = Value::Int(0);
    let result = Value::from_str_with_type(&template, "abc");

    assert!(result.is_err());
}

#[test]
fn test_from_str_with_type_preserves_template() {
    let template = Value::Int(100);
    let result = Value::from_str_with_type(&template, "42");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Int(42));
    // Template should not be modified
    assert_eq!(template, Value::Int(100));
}

#[test]
fn test_value_equality_same_type() {
    assert_eq!(
        Value::Str("test".to_string()),
        Value::Str("test".to_string())
    );
    assert_eq!(Value::Int(42), Value::Int(42));
    assert_eq!(Value::Float(3.14), Value::Float(3.14));
    assert_eq!(Value::Bool(true), Value::Bool(true));
}

#[test]
fn test_value_inequality_different_values() {
    assert_ne!(
        Value::Str("test".to_string()),
        Value::Str("other".to_string())
    );
    assert_ne!(Value::Int(42), Value::Int(43));
    assert_ne!(Value::Bool(true), Value::Bool(false));
}

#[test]
fn test_value_inequality_different_types() {
    assert_ne!(Value::Str("42".to_string()), Value::Int(42));
    assert_ne!(Value::Int(1), Value::Bool(true));
    assert_ne!(Value::Float(3.14), Value::Str("3.14".to_string()));
}
