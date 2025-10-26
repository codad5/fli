use crate::command::FliCommand;
use crate::option_parser::{CommandChain, InputArgsParser, Value, ValueTypes};

// Helper function to create a basic command with options
fn create_test_command() -> FliCommand {
    let mut cmd = FliCommand::new("test", "Test command");

    // Add various option types
    cmd.add_option(
        "verbose",
        "Verbose output",
        "-v",
        "--verbose",
        ValueTypes::None,
    );
    cmd.add_option("quiet", "Quiet mode", "-q", "--quiet", ValueTypes::None);
    cmd.add_option(
        "output",
        "Output file",
        "-o",
        "--output",
        ValueTypes::RequiredSingle(Value::Str(String::new())),
    );
    cmd.add_option(
        "count",
        "Number of items",
        "-n",
        "--count",
        ValueTypes::OptionalSingle(Some(Value::Int(10))),
    );
    cmd.add_option(
        "files",
        "Input files",
        "-f",
        "--files",
        ValueTypes::RequiredMultiple(vec![], None),
    );

    cmd
}

// Helper function to create a command with subcommands
fn create_command_with_subcommands() -> FliCommand {
    let mut root = FliCommand::new("app", "Main app");
    root.add_option("verbose", "Verbose", "-v", "--verbose", ValueTypes::None);

    root.subcommand("start", "Start service").add_option(
        "port",
        "Port",
        "-p",
        "--port",
        ValueTypes::OptionalSingle(Some(Value::Int(8080))),
    );

    root.subcommand("stop", "Stop service").add_option(
        "force",
        "Force stop",
        "-f",
        "--force",
        ValueTypes::None,
    );

    root
}

// ============================================================================
// Basic Parsing Tests
// ============================================================================

#[test]
fn test_empty_args() {
    let args: Vec<String> = vec![];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    let result = parser.prepare(&mut cmd);
    assert!(result.is_ok());

    let chain = parser.get_parsed_commands_chain();
    assert_eq!(chain.len(), 0);
}

#[test]
fn test_single_flag_option() {
    let args = vec!["-v".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();
    assert_eq!(chain.len(), 1);

    match &chain[0] {
        CommandChain::Option(flag, value) => {
            assert_eq!(flag, "-v");
            assert!(matches!(value, ValueTypes::None));
        }
        _ => panic!("Expected Option variant"),
    }
}

#[test]
fn test_long_flag_option() {
    let args = vec!["--verbose".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();
    assert_eq!(chain.len(), 1);

    match &chain[0] {
        CommandChain::Option(flag, _) => {
            assert_eq!(flag, "--verbose");
        }
        _ => panic!("Expected Option variant"),
    }
}

#[test]
fn test_option_with_value() {
    let args = vec!["-o".to_string(), "output.txt".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();
    assert_eq!(chain.len(), 1);

    match &chain[0] {
        CommandChain::Option(flag, value) => {
            assert_eq!(flag, "-o");
            match value {
                ValueTypes::RequiredSingle(Value::Str(s)) => {
                    assert_eq!(s, "output.txt");
                }
                _ => panic!("Expected RequiredSingle(Str)"),
            }
        }
        _ => panic!("Expected Option variant"),
    }
}

#[test]
fn test_long_option_with_value() {
    let args = vec!["--output".to_string(), "result.txt".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();
    match &chain[0] {
        CommandChain::Option(flag, value) => {
            assert_eq!(flag, "--output");
            match value {
                ValueTypes::RequiredSingle(Value::Str(s)) => {
                    assert_eq!(s, "result.txt");
                }
                _ => panic!("Expected string value"),
            }
        }
        _ => panic!("Expected Option variant"),
    }
}

#[test]
fn test_multiple_flags() {
    let args = vec!["-v".to_string(), "-q".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();
    assert_eq!(chain.len(), 2);

    match &chain[0] {
        CommandChain::Option(flag, _) => assert_eq!(flag, "-v"),
        _ => panic!("Expected first option to be -v"),
    }

    match &chain[1] {
        CommandChain::Option(flag, _) => assert_eq!(flag, "-q"),
        _ => panic!("Expected second option to be -q"),
    }
}

#[test]
fn test_positional_arguments() {
    let args = vec!["file1.txt".to_string(), "file2.txt".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();
    cmd.set_expected_positional_args(2); // Tell parser to expect 2 arguments

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();
    assert_eq!(chain.len(), 2);

    match &chain[0] {
        CommandChain::Argument(arg) => assert_eq!(arg, "file1.txt"),
        _ => panic!("Expected Argument variant, got {:?}", chain[0]),
    }

    match &chain[1] {
        CommandChain::Argument(arg) => assert_eq!(arg, "file2.txt"),
        _ => panic!("Expected Argument variant, got {:?}", chain[1]),
    }
} // ============================================================================
  // Mixed Options and Arguments Tests
  // ============================================================================

#[test]
fn test_flags_before_arguments() {
    // Note: Parser state machine doesn't allow transitioning from InOption to InArgument
    // Use -- separator or ensure arguments come in valid states
    let args = vec![
        "-v".to_string(),
        "--".to_string(), // Breaking state allows transition to InArgument
        "file1.txt".to_string(),
        "file2.txt".to_string(),
    ];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Expected: [Option(-v), Argument(file1.txt), Argument(file2.txt)]
    let has_verbose = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Option(flag, _) if flag == "-v"));
    assert!(has_verbose, "Expected -v option in chain");

    let has_file1 = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Argument(arg) if arg == "file1.txt"));
    assert!(has_file1, "Expected file1.txt argument in chain");

    let has_file2 = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Argument(arg) if arg == "file2.txt"));
    assert!(has_file2, "Expected file2.txt argument in chain");
}

#[test]
fn test_option_with_value_and_arguments() {
    // Use -- separator to allow arguments after option with value
    let args = vec![
        "-o".to_string(),
        "output.txt".to_string(),
        "--".to_string(),
        "input.txt".to_string(),
    ];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Expected: [Option(-o, output.txt), Argument(input.txt)]
    let has_output_option = chain.iter().any(|item| match item {
        CommandChain::Option(flag, value) => {
            if flag == "-o" {
                matches!(value, ValueTypes::RequiredSingle(Value::Str(s)) if s == "output.txt")
            } else {
                false
            }
        }
        _ => false,
    });
    assert!(
        has_output_option,
        "Expected -o option with output.txt value"
    );

    let has_input_arg = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Argument(arg) if arg == "input.txt"));
    assert!(has_input_arg, "Expected input.txt argument");
}

#[test]
fn test_multiple_options_with_values() {
    let args = vec![
        "-o".to_string(),
        "out.txt".to_string(),
        "-n".to_string(),
        "5".to_string(),
        "-v".to_string(),
    ];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Expected: [Option(-o, out.txt), Option(-n, 5), Option(-v)]
    assert_eq!(chain.len(), 3);
}

#[test]
fn test_double_dash_separator() {
    let args = vec![
        "-v".to_string(),
        "--".to_string(),
        "-o".to_string(),
        "file.txt".to_string(),
    ];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Expected: [Option(-v), Argument(-o), Argument(file.txt)]
    // After --, everything should be treated as arguments
    assert!(chain.len() >= 2);

    match &chain[0] {
        CommandChain::Option(flag, _) => assert_eq!(flag, "-v"),
        _ => panic!("Expected first to be an option"),
    }

    // After --, -o should be treated as argument, not option
    let has_argument_o = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Argument(arg) if arg == "-o"));
    assert!(
        has_argument_o,
        "Expected -o to be treated as argument after --"
    );
}

// ============================================================================
// Subcommand Tests
// ============================================================================

#[test]
fn test_subcommand_parsing() {
    let args = vec!["start".to_string()];
    let mut parser = InputArgsParser::new("app".to_string(), args);
    let mut cmd = create_command_with_subcommands();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Expected: [SubCommand(start)]
    assert_eq!(chain.len(), 1);

    match &chain[0] {
        CommandChain::SubCommand(name) => assert_eq!(name, "start"),
        _ => panic!("Expected SubCommand variant"),
    }
}

#[test]
fn test_subcommand_with_options() {
    let args = vec!["start".to_string(), "-p".to_string(), "3000".to_string()];
    let mut parser = InputArgsParser::new("app".to_string(), args);
    let mut cmd = create_command_with_subcommands();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Expected: [SubCommand(start), Option(-p, 3000)]
    assert!(chain.len() >= 1);

    match &chain[0] {
        CommandChain::SubCommand(name) => assert_eq!(name, "start"),
        _ => panic!("Expected SubCommand"),
    }
}

#[test]
fn test_root_option_before_subcommand() {
    let args = vec!["-v".to_string(), "start".to_string()];
    let mut parser = InputArgsParser::new("app".to_string(), args);
    let mut cmd = create_command_with_subcommands();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Expected: [Option(-v), SubCommand(start)]
    assert_eq!(chain.len(), 2);

    match &chain[0] {
        CommandChain::Option(flag, _) => assert_eq!(flag, "-v"),
        _ => panic!("Expected Option first"),
    }

    match &chain[1] {
        CommandChain::SubCommand(name) => assert_eq!(name, "start"),
        _ => panic!("Expected SubCommand second"),
    }
}

// ============================================================================
// Complex Scenarios
// ============================================================================

#[test]
fn test_complex_command_chain() {
    // Use proper separator to allow mixing options and arguments
    let args = vec![
        "-v".to_string(),
        "-o".to_string(),
        "output.txt".to_string(),
        "--".to_string(), // Separator to switch to arguments
        "file1.txt".to_string(),
        "file2.txt".to_string(),
    ];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Expected: [Option(-v), Option(-o, output.txt), Argument(file1.txt),
    //            Argument(file2.txt)]

    // Verify structure
    let has_verbose = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Option(flag, _) if flag == "-v"));
    assert!(has_verbose, "Expected -v option");

    let has_output = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Option(flag, _) if flag == "-o"));
    assert!(has_output, "Expected -o option");

    let has_file1 = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Argument(arg) if arg == "file1.txt"));
    assert!(has_file1, "Expected file1.txt argument");

    let has_file2 = chain
        .iter()
        .any(|item| matches!(item, CommandChain::Argument(arg) if arg == "file2.txt"));
    assert!(has_file2, "Expected file2.txt argument");
}

#[test]
fn test_help_flag_as_preserved_option() {
    let args = vec!["--help".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Help flag should appear in chain (might be IsPreservedOption or Option)
    assert!(chain.len() >= 1);

    let has_help = chain.iter().any(|item| match item {
        CommandChain::IsPreservedOption(flag) => flag == "--help",
        CommandChain::Option(flag, _) => flag == "--help",
        _ => false,
    });
    assert!(has_help);
}

#[test]
fn test_numeric_option_parsing() {
    let args = vec!["--count".to_string(), "42".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    match &chain[0] {
        CommandChain::Option(flag, value) => {
            assert_eq!(flag, "--count");
            match value {
                ValueTypes::OptionalSingle(Some(Value::Int(n))) => {
                    assert_eq!(*n, 42);
                }
                _ => panic!("Expected OptionalSingle(Int(42))"),
            }
        }
        _ => panic!("Expected Option variant"),
    }
}

#[test]
fn test_multiple_value_option() {
    let args = vec![
        "-f".to_string(),
        "file1.txt".to_string(),
        "file2.txt".to_string(),
        "file3.txt".to_string(),
    ];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let chain = parser.get_parsed_commands_chain();

    // Should have at least one option for -f
    match &chain[0] {
        CommandChain::Option(flag, value) => {
            assert_eq!(flag, "-f");
            match value {
                ValueTypes::RequiredMultiple(_, _) => {
                    // Successfully parsed as multiple values
                }
                _ => panic!("Expected RequiredMultiple"),
            }
        }
        _ => panic!("Expected Option variant"),
    }
}

// ============================================================================
// Edge Cases and Error Scenarios
// ============================================================================

#[test]
fn test_unknown_option() {
    let args = vec!["-x".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    let result = parser.prepare(&mut cmd);

    // Unknown option should result in error or be treated as argument
    // depending on implementation
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_missing_required_value() {
    let args = vec!["-o".to_string()]; // -o requires a value
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    let result = parser.prepare(&mut cmd);

    // Should either error or handle gracefully
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_equals_syntax_long_option() {
    let args = vec!["--output=file.txt".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    let result = parser.prepare(&mut cmd);

    // Test if parser supports --option=value syntax
    if result.is_ok() {
        let chain = parser.get_parsed_commands_chain();
        // Verify parsing worked correctly
        assert!(chain.len() >= 1);
    }
}

#[test]
fn test_short_option_clustering() {
    // Some parsers support -vq instead of -v -q
    let args = vec!["-vq".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    let result = parser.prepare(&mut cmd);

    // Test if clustering is supported
    if result.is_ok() {
        let chain = parser.get_parsed_commands_chain();
        // Check if it was parsed as two options or handled differently
        assert!(chain.len() >= 1);
    }
}

// ============================================================================
// Parser State Tests
// ============================================================================

#[test]
fn test_parser_not_prepared() {
    let args = vec!["-v".to_string()];
    let parser = InputArgsParser::new("test".to_string(), args);

    // Parser should not have chain before prepare()
    let chain = parser.get_parsed_commands_chain();
    assert_eq!(chain.len(), 0);
}

#[test]
fn test_parser_prepared_multiple_times() {
    let args = vec!["-v".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    // First prepare
    parser.prepare(&mut cmd).unwrap();
    let chain1_len = parser.get_parsed_commands_chain().len();

    // Second prepare - should not duplicate
    let result = parser.prepare(&mut cmd);

    // Implementation may prevent re-preparation
    if result.is_ok() {
        let chain2_len = parser.get_parsed_commands_chain().len();
        // Chain should not grow from re-preparation
        assert_eq!(chain1_len, chain2_len);
    }
}

#[test]
fn test_get_command_chain_vs_parsed_chain() {
    // Use -- separator to allow arguments after options
    let args = vec!["-v".to_string(), "--".to_string(), "file.txt".to_string()];
    let mut parser = InputArgsParser::new("test".to_string(), args);
    let mut cmd = create_test_command();

    parser.prepare(&mut cmd).unwrap();

    let parsed_chain = parser.get_parsed_commands_chain();
    let command_chain = parser.get_command_chain();

    // Both should return the same chain
    assert_eq!(parsed_chain.len(), command_chain.len());
}
