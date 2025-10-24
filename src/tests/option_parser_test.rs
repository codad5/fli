#[cfg(test)]
mod tests {
    use crate::command::FliCommand;
    use crate::option_parser::{CommandChain, InputArgsParser, Value, ValueTypes};

    fn create_test_command() -> FliCommand {
        let mut cmd = FliCommand::new("test", "Test command");
        
        // Add various option types
        cmd.add_option("output", "Output file", "-o", "--output", 
            ValueTypes::RequiredSingle(Value::Str(String::new())));
        cmd.add_option("verbose", "Verbose mode", "-v", "--verbose", 
            ValueTypes::None);
        cmd.add_option("input", "Input files", "-i", "--input", 
            ValueTypes::RequiredMultiple(vec![], Some(2)));
        cmd.add_option("config", "Config file", "-c", "--config", 
            ValueTypes::OptionalSingle(None));
        cmd.add_option("files", "Additional files", "-f", "--files", 
            ValueTypes::OptionalMultiple(None, None));
        
        cmd
    }

    #[test]
    fn test_simple_command_with_arguments() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["arg1".to_string(), "arg2".to_string()]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        assert_eq!(chain.len(), 2);
        assert!(matches!(chain[0], CommandChain::Argument(_)));
        assert!(matches!(chain[1], CommandChain::Argument(_)));
    }

    #[test]
    fn test_args_then_options_allowed() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["arg1".to_string(), "arg2".to_string(), "-v".to_string()]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        assert_eq!(chain.len(), 3);
        assert!(matches!(chain[0], CommandChain::Argument(_)));
        assert!(matches!(chain[1], CommandChain::Argument(_)));
        assert!(matches!(chain[2], CommandChain::Option(_, ValueTypes::None)));
    }

    #[test]
    fn test_options_then_args_without_break_fails() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["-v".to_string(), "arg1".to_string()]
        );
        
        // This should fail because InOption cannot go to InArgument without break
        assert!(parser.prepare(&cmd).is_err());
    }

    #[test]
    fn test_options_then_break_then_args_succeeds() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["-v".to_string(), "--".to_string(), "arg1".to_string(), "-v".to_string()]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        assert_eq!(chain.len(), 3);
        assert!(matches!(chain[0], CommandChain::Option(_, ValueTypes::None)));
        // arg1 is an argument
        assert!(matches!(chain[1], CommandChain::Argument(_)));
        // -v after break is treated as argument, not option
        assert!(matches!(chain[2], CommandChain::Argument(_)));
    }

    #[test]
    fn test_required_single_value() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["-o".to_string(), "output.txt".to_string()]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        assert_eq!(chain.len(), 1);
        
        if let CommandChain::Option(name, ValueTypes::RequiredSingle(Value::Str(val))) = &chain[0] {
            assert_eq!(name, "-o");
            assert_eq!(val, "output.txt");
        } else {
            panic!("Expected RequiredSingle option");
        }
    }

    #[test]
    fn test_required_single_missing_value() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["-o".to_string()]
        );
        
        let result = parser.prepare(&cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing required value"));
    }

    #[test]
    fn test_required_multiple_values() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["-i".to_string(), "file1.txt".to_string(), "file2.txt".to_string()]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        assert_eq!(chain.len(), 1);
        
        if let CommandChain::Option(name, ValueTypes::RequiredMultiple(vals, count)) = &chain[0] {
            assert_eq!(name, "-i");
            assert_eq!(vals.len(), 2);
            assert_eq!(*count, Some(2));
        } else {
            panic!("Expected RequiredMultiple option");
        }
    }

    #[test]
    fn test_optional_single_with_value() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["-c".to_string(), "config.json".to_string()]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        if let CommandChain::Option(_, ValueTypes::OptionalSingle(Some(Value::Str(val)))) = &chain[0] {
            assert_eq!(val, "config.json");
        } else {
            panic!("Expected OptionalSingle with value");
        }
    }

    #[test]
    fn test_optional_single_without_value() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["-c".to_string()]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        if let CommandChain::Option(_, ValueTypes::OptionalSingle(None)) = &chain[0] {
            // Success
        } else {
            panic!("Expected OptionalSingle with None");
        }
    }

    #[test]
    fn test_chaining_options() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec!["-v".to_string(), "-o".to_string(), "out.txt".to_string(), "-v".to_string()]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn test_mixed_pattern_with_break() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "test".to_string(),
            vec![
                "-v".to_string(),
                "-o".to_string(),
                "output.txt".to_string(),
                "--".to_string(),
                "arg1".to_string(),
                "--config".to_string(), // This is now an argument, not an option
            ]
        );
        
        assert!(parser.prepare(&cmd).is_ok());
        
        let chain = parser.get_parsed_commands_chain();
        // -v, -o output.txt, arg1, --config (as arg)
        assert_eq!(chain.len(), 4);
    }

    #[test]
    fn test_command_name_mismatch() {
        let cmd = create_test_command();
        let mut parser = InputArgsParser::new(
            "wrong".to_string(),
            vec![]
        );
        
        let result = parser.prepare(&cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Command name mismatch"));
    }
}