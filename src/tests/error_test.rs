use crate::error::FliError;

#[test]
fn test_command_mismatch_error() {
    let error = FliError::CommandMismatch {
        expected: "serve".to_string(),
        actual: "run".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "Command mismatch: expected 'serve', got 'run'"
    );
}

#[test]
fn test_unknown_command_error() {
    let error = FliError::UnknownCommand(
        "serv".to_string(),
        vec!["serve".to_string(), "start".to_string()],
    );

    let error_msg = error.to_string();
    assert!(error_msg.contains("Unknown command: 'serv'"));
}

#[test]
fn test_unknown_option_error() {
    let error = FliError::UnknownOption("--verbos".to_string());

    assert_eq!(
        error.to_string(),
        "Unknown option: '--verbos'. Run with --help to see available options"
    );
}

#[test]
fn test_missing_value_error() {
    let error = FliError::MissingValue {
        option: "--output".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "Missing required value for option '--output'"
    );
}

#[test]
fn test_unexpected_value_error() {
    let error = FliError::UnexpectedValue {
        option: "--verbose".to_string(),
        value: "yes".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "Option '--verbose' does not accept values, but 'yes' was provided"
    );
}

#[test]
fn test_value_count_mismatch_error() {
    let error = FliError::ValueCountMismatch {
        option: "--files".to_string(),
        expected: 3,
        actual: 2,
    };

    assert_eq!(
        error.to_string(),
        "Option '--files' expected 3 value(s), got 2"
    );
}

#[test]
fn test_invalid_value_error() {
    let error = FliError::InvalidValue {
        option: "--port".to_string(),
        value: "abc".to_string(),
        reason: "not a valid integer".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "Invalid value 'abc' for option '--port': not a valid integer"
    );
}

#[test]
fn test_invalid_flag_format_error() {
    let error = FliError::InvalidFlagFormat {
        flag: "verbose".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "Invalid flag format: 'verbose'. Flags must start with '-' or '--'"
    );
}

#[test]
fn test_option_not_found_error() {
    let error = FliError::OptionNotFound("--config".to_string());

    assert_eq!(error.to_string(), "Option '--config' not found in parser");
}

#[test]
fn test_parser_not_prepared_error() {
    let error = FliError::ParserNotPrepared;

    assert_eq!(
        error.to_string(),
        "Parser must be prepared before use. Call prepare() first"
    );
}

#[test]
fn test_internal_error() {
    let error = FliError::Internal("Something went wrong".to_string());

    assert_eq!(error.to_string(), "Internal error: Something went wrong");
}

#[test]
fn test_error_helper_functions() {
    let error = FliError::command_mismatch("expected", "actual");
    assert!(matches!(error, FliError::CommandMismatch { .. }));

    let error = FliError::missing_value("--name");
    assert!(matches!(error, FliError::MissingValue { .. }));
}
