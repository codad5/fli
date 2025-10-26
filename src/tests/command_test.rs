use crate::command::{FliCallbackData, FliCommand};
use crate::option_parser::{InputArgsParser, Value, ValueTypes};

#[test]
fn test_fli_command_creation() {
    let cmd = FliCommand::new("test", "A test command");
    assert_eq!(cmd.get_name(), "test");
    assert_eq!(cmd.get_description(), "A test command");
}

#[test]
fn test_add_sub_command() {
    let mut parent = FliCommand::new("parent", "Parent command");
    let child = FliCommand::new("child", "Child command");

    parent.add_sub_command(child);
    assert!(parent.has_sub_command("child"));
}

#[test]
fn test_has_sub_command() {
    let mut cmd = FliCommand::new("main", "Main command");
    let sub = FliCommand::new("sub", "Sub command");

    assert!(!cmd.has_sub_command("sub"));
    cmd.add_sub_command(sub);
    assert!(cmd.has_sub_command("sub"));
}

#[test]
fn test_get_sub_command() {
    let mut parent = FliCommand::new("parent", "Parent command");
    let child = FliCommand::new("child", "Child command");

    parent.add_sub_command(child);

    let retrieved = parent.get_sub_command("child");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().get_name(), "child");
}

#[test]
fn test_get_nonexistent_sub_command() {
    let cmd = FliCommand::new("main", "Main command");
    assert!(cmd.get_sub_command("nonexistent").is_none());
}

#[test]
fn test_add_option_to_command() {
    let mut cmd = FliCommand::new("test", "Test command");

    cmd.add_option(
        "verbose",
        "Enable verbose output",
        "-v",
        "--verbose",
        ValueTypes::None,
    );

    let parser = cmd.get_option_parser();
    assert!(parser.has_option("-v"));
    assert!(parser.has_option("--verbose"));
}

#[test]
fn test_multiple_options() {
    let mut cmd = FliCommand::new("build", "Build command");

    cmd.add_option(
        "release",
        "Build in release mode",
        "-r",
        "--release",
        ValueTypes::None,
    );

    cmd.add_option(
        "target",
        "Target directory",
        "-t",
        "--target",
        ValueTypes::RequiredSingle(Value::Str(String::new())),
    );

    let parser = cmd.get_option_parser();
    // 2 options + 1 default help option = 3
    assert_eq!(parser.get_options().len(), 3);
    assert!(parser.has_option("-r"));
    assert!(parser.has_option("--target"));
}

#[test]
fn test_callback_data_creation() {
    let mut cmd = FliCommand::new("test", "Test");
    let parser = cmd.get_option_parser().clone();
    let args = vec!["arg1".to_string(), "arg2".to_string()];
    let arg_parser = InputArgsParser::new("test".to_string(), vec![]);

    let data = FliCallbackData::new(cmd.clone(), parser, args.clone(), arg_parser);

    assert_eq!(data.command.get_name(), "test");
    assert_eq!(data.arguments.len(), 2);
    assert_eq!(data.arguments[0], "arg1");
}

#[test]
fn test_callback_data_get_option_value() {
    let mut cmd = FliCommand::new("test", "Test");
    cmd.add_option(
        "name",
        "Name option",
        "-n",
        "--name",
        ValueTypes::RequiredSingle(Value::Str("default".to_string())),
    );

    let mut parser = cmd.get_option_parser().clone();
    parser
        .update_option_value(
            "-n",
            ValueTypes::RequiredSingle(Value::Str("Alice".to_string())),
        )
        .unwrap();

    let arg_parser = InputArgsParser::new("test".to_string(), vec![]);
    let data = FliCallbackData::new(cmd.clone(), parser, vec![], arg_parser);

    let value = data.get_option_value("name");
    assert!(value.is_some());
    assert_eq!(value.unwrap().as_str(), Some("Alice"));
}

#[test]
fn test_callback_data_get_option_with_dash() {
    let mut cmd = FliCommand::new("test", "Test");
    cmd.add_option(
        "verbose",
        "Verbose mode",
        "-v",
        "--verbose",
        ValueTypes::None,
    );

    let parser = cmd.get_option_parser().clone();
    let arg_parser = InputArgsParser::new("test".to_string(), vec![]);
    let data = FliCallbackData::new(cmd, parser, vec![], arg_parser);

    // Should work with both formats
    let value1 = data.get_option_value("-v");
    let value2 = data.get_option_value("--verbose");
    let value3 = data.get_option_value("verbose");

    assert!(value1.is_some() || value2.is_some() || value3.is_some());
}

#[test]
fn test_nested_subcommands() {
    let mut root = FliCommand::new("git", "Git command");
    let mut remote = FliCommand::new("remote", "Remote commands");
    let add = FliCommand::new("add", "Add remote");

    remote.add_sub_command(add);
    root.add_sub_command(remote);

    assert!(root.has_sub_command("remote"));
    let remote_cmd = root.get_sub_command("remote").unwrap();
    assert!(remote_cmd.has_sub_command("add"));
}

#[test]
fn test_command_clone() {
    let mut cmd = FliCommand::new("test", "Test command");
    cmd.add_option("opt", "Option", "-o", "--opt", ValueTypes::None);

    let mut cloned = cmd.clone();
    assert_eq!(cloned.get_name(), cmd.get_name());
    assert!(cloned.get_option_parser().has_option("-o"));
}

// ============================================================================
// Inheritable Options Tests (Command Level)
// ============================================================================

#[test]
fn test_command_with_parser_constructor() {
    use crate::option_parser::CommandOptionsParserBuilder;

    let mut builder = CommandOptionsParserBuilder::new();
    builder.add_option("test", "Test option", "-t", "--test", ValueTypes::None);

    let mut cmd = FliCommand::with_parser("mycmd", "My command", builder);

    assert_eq!(cmd.get_name(), "mycmd");
    assert_eq!(cmd.get_description(), "My command");
    assert!(cmd.get_option_parser().has_option("-t"));
    // Help flag should be auto-added
    assert!(cmd.get_option_parser().has_option("--help"));
}

#[test]
fn test_subcommand_inherits_parent_options() {
    let mut parent = FliCommand::new("parent", "Parent command");

    // Add options to parent
    parent.add_option(
        "verbose",
        "Verbose output",
        "-v",
        "--verbose",
        ValueTypes::None,
    );
    parent.add_option("debug", "Debug mode", "-d", "--debug", ValueTypes::None);

    // Mark options as inheritable
    parent.get_option_parser().mark_inheritable("-v").unwrap();
    parent.get_option_parser().mark_inheritable("-d").unwrap();

    // Create subcommand using subcommand() method
    let child = parent.subcommand("child", "Child command");

    // Child should have inherited options
    assert!(child.get_option_parser().has_option("-v"));
    assert!(child.get_option_parser().has_option("--verbose"));
    assert!(child.get_option_parser().has_option("-d"));
    assert!(child.get_option_parser().has_option("--debug"));
}

#[test]
fn test_subcommand_inherits_only_marked_options() {
    let mut parent = FliCommand::new("parent", "Parent command");

    // Add multiple options
    parent.add_option("verbose", "Verbose", "-v", "--verbose", ValueTypes::None);
    parent.add_option("quiet", "Quiet", "-q", "--quiet", ValueTypes::None);
    parent.add_option("debug", "Debug", "-d", "--debug", ValueTypes::None);

    // Mark only verbose as inheritable
    parent.get_option_parser().mark_inheritable("-v").unwrap();

    // Create subcommand
    let child = parent.subcommand("child", "Child command");

    // Child should have only the verbose option, not quiet or debug
    assert!(child.get_option_parser().has_option("-v"));
    assert!(!child.get_option_parser().has_option("-q"));
    assert!(!child.get_option_parser().has_option("-d"));
}

#[test]
fn test_nested_subcommands_inherit_options() {
    let mut root = FliCommand::new("root", "Root command");

    // Add option to root
    root.add_option("verbose", "Verbose", "-v", "--verbose", ValueTypes::None);
    root.get_option_parser().mark_inheritable("-v").unwrap();

    // Create first level subcommand
    let level1 = root.subcommand("level1", "Level 1");
    level1.add_option("debug", "Debug", "-d", "--debug", ValueTypes::None);
    level1
        .get_option_parser()
        .mark_inheritable_many(&["-v", "-d"])
        .unwrap();

    // Create second level subcommand
    let level2 = level1.subcommand("level2", "Level 2");

    // Level 2 should have both -v (from root) and -d (from level1)
    assert!(level2.get_option_parser().has_option("-v"));
    assert!(level2.get_option_parser().has_option("-d"));
}

#[test]
fn test_mark_inheritable_with_long_flag() {
    let mut parent = FliCommand::new("parent", "Parent");

    parent.add_option("verbose", "Verbose", "-v", "--verbose", ValueTypes::None);

    // Mark using long flag
    parent
        .get_option_parser()
        .mark_inheritable("--verbose")
        .unwrap();

    let child = parent.subcommand("child", "Child");

    // Should work with both short and long flags
    assert!(child.get_option_parser().has_option("-v"));
    assert!(child.get_option_parser().has_option("--verbose"));
}

#[test]
fn test_mark_inheritable_many_mixed_flags() {
    let mut parent = FliCommand::new("parent", "Parent");

    parent.add_option("verbose", "Verbose", "-v", "--verbose", ValueTypes::None);
    parent.add_option("quiet", "Quiet", "-q", "--quiet", ValueTypes::None);
    parent.add_option("debug", "Debug", "-d", "--debug", ValueTypes::None);

    // Mix short and long flags
    parent
        .get_option_parser()
        .mark_inheritable_many(&["-v", "--quiet", "-d"])
        .unwrap();

    let child = parent.subcommand("child", "Child");

    assert!(child.get_option_parser().has_option("-v"));
    assert!(child.get_option_parser().has_option("-q"));
    assert!(child.get_option_parser().has_option("-d"));
}

#[test]
fn test_inheritable_options_with_default_values() {
    let mut parent = FliCommand::new("parent", "Parent");

    parent.add_option(
        "config",
        "Config file",
        "-c",
        "--config",
        ValueTypes::OptionalSingle(Some(Value::Str("default.toml".to_string()))),
    );

    parent.get_option_parser().mark_inheritable("-c").unwrap();

    let child = parent.subcommand("child", "Child");

    // Child should inherit the option with its default value
    assert!(child.get_option_parser().has_option("-c"));
}

#[test]
fn test_inheritable_options_builder_creates_independent_copy() {
    let mut parent = FliCommand::new("parent", "Parent");

    parent.add_option("verbose", "Verbose", "-v", "--verbose", ValueTypes::None);
    parent.get_option_parser().mark_inheritable("-v").unwrap();

    // Create child1 and add option to it
    {
        let child1 = parent.subcommand("child1", "Child 1");
        child1.add_option("opt1", "Option 1", "-o", "--opt1", ValueTypes::None);
        // child1 should have both inherited and its own option
        assert!(child1.get_option_parser().has_option("-v"));
        assert!(child1.get_option_parser().has_option("-o"));
    }

    // Create child2 separately
    let child2 = parent.subcommand("child2", "Child 2");

    // child2 should not have child1's option
    assert!(!child2.get_option_parser().has_option("-o"));
    // But should have inherited option
    assert!(child2.get_option_parser().has_option("-v"));
}

#[test]
fn test_mark_nonexistent_option_as_inheritable() {
    let mut cmd = FliCommand::new("test", "Test");

    // Try to mark an option that doesn't exist
    let result = cmd.get_option_parser().mark_inheritable("-v");

    assert!(result.is_err());
}

#[test]
fn test_subcommand_adds_help_flag_to_inherited_options() {
    let mut parent = FliCommand::new("parent", "Parent");

    parent.add_option("verbose", "Verbose", "-v", "--verbose", ValueTypes::None);
    parent.get_option_parser().mark_inheritable("-v").unwrap();

    let child = parent.subcommand("child", "Child");

    // Child should have both inherited option and auto-added help
    assert!(child.get_option_parser().has_option("-v"));
    assert!(child.get_option_parser().has_option("--help"));
}
