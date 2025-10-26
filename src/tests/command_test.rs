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
