use crate::option_parser::{
    CommandOptionsParser, CommandOptionsParserBuilder, SingleOption, Value, ValueTypes,
};

#[test]
fn test_single_option_creation() {
    let option = SingleOption {
        name: "verbose".to_string(),
        description: "Enable verbose output".to_string(),
        short_flag: "-v".to_string(),
        long_flag: "--verbose".to_string(),
        value: ValueTypes::None,
    };

    assert_eq!(option.name, "verbose");
    assert_eq!(option.short_flag, "-v");
    assert_eq!(option.long_flag, "--verbose");
}

#[test]
fn test_command_options_parser_new() {
    let parser = CommandOptionsParser::new();
    assert_eq!(parser.get_options().len(), 0);
}

#[test]
fn test_add_option() {
    let mut parser = CommandOptionsParser::new();
    let option = SingleOption {
        name: "output".to_string(),
        description: "Output file".to_string(),
        short_flag: "-o".to_string(),
        long_flag: "--output".to_string(),
        value: ValueTypes::RequiredSingle(Value::Str(String::new())),
    };

    parser.add_option(option);
    assert_eq!(parser.get_options().len(), 1);
}

#[test]
fn test_get_option_by_short_flag() {
    let mut parser = CommandOptionsParser::new();
    let option = SingleOption {
        name: "verbose".to_string(),
        description: "Enable verbose".to_string(),
        short_flag: "-v".to_string(),
        long_flag: "--verbose".to_string(),
        value: ValueTypes::None,
    };

    parser.add_option(option);

    let retrieved = parser.get_option_by_short_flag("-v");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "verbose");
}

#[test]
fn test_get_option_by_long_flag() {
    let mut parser = CommandOptionsParser::new();
    let option = SingleOption {
        name: "config".to_string(),
        description: "Config file".to_string(),
        short_flag: "-c".to_string(),
        long_flag: "--config".to_string(),
        value: ValueTypes::OptionalSingle(None),
    };

    parser.add_option(option);

    let retrieved = parser.get_option_by_long_flag("--config");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "config");
}

#[test]
fn test_has_option() {
    let mut parser = CommandOptionsParser::new();
    let option = SingleOption {
        name: "help".to_string(),
        description: "Show help".to_string(),
        short_flag: "-h".to_string(),
        long_flag: "--help".to_string(),
        value: ValueTypes::None,
    };

    parser.add_option(option);

    assert!(parser.has_option("-h"));
    assert!(parser.has_option("--help"));
    assert!(!parser.has_option("-x"));
    assert!(!parser.has_option("--unknown"));
}

#[test]
fn test_update_option_value() {
    let mut parser = CommandOptionsParser::new();
    let option = SingleOption {
        name: "port".to_string(),
        description: "Port number".to_string(),
        short_flag: "-p".to_string(),
        long_flag: "--port".to_string(),
        value: ValueTypes::RequiredSingle(Value::Int(8080)),
    };

    parser.add_option(option);

    let result = parser.update_option_value(
        "-p",
        ValueTypes::RequiredSingle(Value::Int(3000)),
    );
    assert!(result.is_ok());

    let updated = parser.get_option_by_short_flag("-p").unwrap();
    match &updated.value {
        ValueTypes::RequiredSingle(Value::Int(n)) => assert_eq!(*n, 3000),
        _ => panic!("Expected Int value"),
    }
}

#[test]
fn test_update_nonexistent_option() {
    let mut parser = CommandOptionsParser::new();
    
    let result = parser.update_option_value(
        "-x",
        ValueTypes::None,
    );
    assert!(result.is_err());
}

#[test]
fn test_get_option_expected_value_type() {
    let mut parser = CommandOptionsParser::new();
    let option = SingleOption {
        name: "files".to_string(),
        description: "Input files".to_string(),
        short_flag: "-f".to_string(),
        long_flag: "--files".to_string(),
        value: ValueTypes::RequiredMultiple(vec![], None),
    };

    parser.add_option(option);

    let value_type = parser.get_option_expected_value_type("-f");
    assert!(value_type.is_some());
    assert!(value_type.unwrap().expects_value());
}

#[test]
fn test_builder_pattern() {
    let mut builder = CommandOptionsParserBuilder::new();
    builder
        .add_option(
            "verbose",
            "Enable verbose output",
            "-v",
            "--verbose",
            ValueTypes::None,
        )
        .add_option(
            "output",
            "Output file",
            "-o",
            "--output",
            ValueTypes::RequiredSingle(Value::Str(String::new())),
        );

    let parser = builder.build();
    assert_eq!(parser.get_options().len(), 2);
    assert!(parser.has_option("-v"));
    assert!(parser.has_option("--output"));
}

#[test]
fn test_builder_chaining() {
    let mut builder = CommandOptionsParserBuilder::new();
    let parser = builder
        .add_option("debug", "Debug mode", "-d", "--debug", ValueTypes::None)
        .add_option(
            "level",
            "Log level",
            "-l",
            "--level",
            ValueTypes::RequiredSingle(Value::Str("info".to_string())),
        )
        .build();

    assert_eq!(parser.get_options().len(), 2);
}

#[test]
fn test_multiple_options_different_flags() {
    let mut parser = CommandOptionsParser::new();

    parser.add_option(SingleOption {
        name: "opt1".to_string(),
        description: "Option 1".to_string(),
        short_flag: "-a".to_string(),
        long_flag: "--alpha".to_string(),
        value: ValueTypes::None,
    });

    parser.add_option(SingleOption {
        name: "opt2".to_string(),
        description: "Option 2".to_string(),
        short_flag: "-b".to_string(),
        long_flag: "--beta".to_string(),
        value: ValueTypes::None,
    });

    assert_eq!(parser.get_options().len(), 2);
    assert!(parser.has_option("-a"));
    assert!(parser.has_option("-b"));
    assert!(parser.has_option("--alpha"));
    assert!(parser.has_option("--beta"));
}

#[test]
fn test_parser_clone() {
    let mut parser = CommandOptionsParser::new();
    parser.add_option(SingleOption {
        name: "test".to_string(),
        description: "Test option".to_string(),
        short_flag: "-t".to_string(),
        long_flag: "--test".to_string(),
        value: ValueTypes::None,
    });

    let cloned = parser.clone();
    assert_eq!(cloned.get_options().len(), parser.get_options().len());
    assert!(cloned.has_option("-t"));
}
