use crate::app::Fli;
use crate::option_parser::{Value, ValueTypes};

#[test]
fn test_fli_new() {
    let app = Fli::new("myapp", "1.0.0", "My application");

    assert_eq!(app.name, "myapp");
    assert_eq!(app.version, "1.0.0");
    assert_eq!(app.description, "My application");
}

#[test]
fn test_add_command() {
    use crate::command::FliCommand;

    let mut app = Fli::new("cli", "1.0.0", "CLI app");
    let cmd = FliCommand::new("serve", "Start server");

    app.add_command(cmd);
    assert!(app.root_command.has_sub_command("serve"));
}

#[test]
fn test_command_method() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    let result = app.command("build", "Build the project");
    assert!(result.is_ok());

    let cmd = result.unwrap();
    assert_eq!(cmd.get_name(), "build");
}

#[test]
fn test_add_option_to_app() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    app.add_option(
        "verbose",
        "Enable verbose output",
        "-v",
        "--verbose",
        ValueTypes::None,
    );

    assert!(app.root_command.get_option_parser().has_option("-v"));
    assert!(app.root_command.get_option_parser().has_option("--verbose"));
}

#[test]
fn test_add_multiple_options() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    app.add_option("verbose", "Verbose", "-v", "--verbose", ValueTypes::None);
    app.add_option(
        "output",
        "Output file",
        "-o",
        "--output",
        ValueTypes::RequiredSingle(Value::Str(String::new())),
    );
    app.add_option("quiet", "Quiet mode", "-q", "--quiet", ValueTypes::None);

    let parser = app.root_command.get_option_parser();
    // 3 options + 1 default help option = 4
    assert_eq!(parser.get_options().len(), 4);
    assert!(parser.has_option("-v"));
    assert!(parser.has_option("--output"));
    assert!(parser.has_option("-q"));
}

#[test]
fn test_command_with_options() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    let cmd = app.command("serve", "Start server").unwrap();
    cmd.add_option(
        "port",
        "Port to listen on",
        "-p",
        "--port",
        ValueTypes::RequiredSingle(Value::Int(8080)),
    );

    let serve_cmd = app.root_command.get_sub_command_mut("serve").unwrap();
    assert!(serve_cmd.get_option_parser().has_option("-p"));
}

#[test]
fn test_nested_commands() {
    let mut app = Fli::new("docker", "1.0.0", "Docker CLI");

    app.command("container", "Manage containers").unwrap();

    let container_cmd = app.root_command.get_sub_command("container").unwrap();
    assert_eq!(container_cmd.get_name(), "container");
}

#[test]
fn test_with_debug() {
    use crate::display;

    let _app = Fli::new("cli", "1.0.0", "CLI app").with_debug();

    assert!(display::is_debug_enabled());
    display::disable_debug(); // Clean up
}

#[test]
fn test_add_debug_option() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");
    app.add_debug_option();

    assert!(app.root_command.get_option_parser().has_option("-D"));
    assert!(app.root_command.get_option_parser().has_option("--debug"));
}

#[test]
fn test_multiple_commands() {
    let mut app = Fli::new("git", "2.0.0", "Git CLI");

    app.command("clone", "Clone repository").unwrap();
    app.command("pull", "Pull changes").unwrap();
    app.command("push", "Push changes").unwrap();

    assert!(app.root_command.has_sub_command("clone"));
    assert!(app.root_command.has_sub_command("pull"));
    assert!(app.root_command.has_sub_command("push"));
}

#[test]
fn test_command_chaining() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    let result = app.command("build", "Build project").and_then(|cmd| {
        cmd.add_option(
            "release",
            "Release mode",
            "-r",
            "--release",
            ValueTypes::None,
        );
        Ok(cmd)
    });

    assert!(result.is_ok());
}

#[test]
fn test_root_command_has_no_name() {
    let app = Fli::new("cli", "1.0.0", "CLI app");
    // Root command should have empty string as name
    assert_eq!(app.root_command.get_name(), "");
}

#[test]
fn test_app_clone_fields() {
    let app = Fli::new("myapp", "2.0.0", "My awesome app");

    assert_eq!(app.name, "myapp");
    assert_eq!(app.version, "2.0.0");
    assert_eq!(app.description, "My awesome app");
}

// ============================================================================
// Inheritable Options Tests
// ============================================================================

#[test]
fn test_mark_inheritable_single_option() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add an option
    app.add_option(
        "verbose",
        "Enable verbose",
        "-v",
        "--verbose",
        ValueTypes::None,
    );

    // Mark it as inheritable
    let result = app.mark_inheritable("-v");
    assert!(result.is_ok());
}

#[test]
fn test_mark_inheritable_nonexistent_option() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Try to mark a non-existent option
    let result = app.mark_inheritable("-v");
    assert!(result.is_err());
}

#[test]
fn test_mark_inheritable_many_options() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add multiple options
    app.add_option(
        "verbose",
        "Enable verbose",
        "-v",
        "--verbose",
        ValueTypes::None,
    );
    app.add_option(
        "quiet",
        "Suppress output",
        "-q",
        "--quiet",
        ValueTypes::None,
    );
    app.add_option("color", "Enable colors", "-c", "--color", ValueTypes::None);

    // Mark them all as inheritable
    let result = app.mark_inheritable_many(&["-v", "-q", "-c"]);
    assert!(result.is_ok());
}

#[test]
fn test_mark_inheritable_many_with_invalid_option() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add only one option
    app.add_option(
        "verbose",
        "Enable verbose",
        "-v",
        "--verbose",
        ValueTypes::None,
    );

    // Try to mark multiple, including non-existent ones
    let result = app.mark_inheritable_many(&["-v", "-q", "-c"]);
    assert!(result.is_err());
}

#[test]
fn test_subcommand_inherits_options() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add options to root
    app.add_option(
        "verbose",
        "Enable verbose",
        "-v",
        "--verbose",
        ValueTypes::None,
    );
    app.add_option(
        "quiet",
        "Suppress output",
        "-q",
        "--quiet",
        ValueTypes::None,
    );

    // Mark them as inheritable
    app.mark_inheritable_many(&["-v", "-q"]).unwrap();

    // Create a subcommand
    let result = app.command("build", "Build project");
    assert!(result.is_ok());

    let cmd = result.unwrap();

    // Check that the subcommand has the inherited options
    assert!(cmd.get_option_parser().has_option("-v"));
    assert!(cmd.get_option_parser().has_option("--verbose"));
    assert!(cmd.get_option_parser().has_option("-q"));
    assert!(cmd.get_option_parser().has_option("--quiet"));
}

#[test]
fn test_multiple_subcommands_inherit_same_options() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add and mark options as inheritable
    app.add_option(
        "verbose",
        "Enable verbose",
        "-v",
        "--verbose",
        ValueTypes::None,
    );
    app.mark_inheritable("-v").unwrap();

    // Create multiple subcommands
    app.command("build", "Build project").unwrap();
    app.command("test", "Run tests").unwrap();
    app.command("deploy", "Deploy app").unwrap();

    // All should have the verbose option
    assert!(app
        .root_command
        .get_sub_command_mut("build")
        .unwrap()
        .get_option_parser()
        .has_option("-v"));
    assert!(app
        .root_command
        .get_sub_command_mut("test")
        .unwrap()
        .get_option_parser()
        .has_option("-v"));
    assert!(app
        .root_command
        .get_sub_command_mut("deploy")
        .unwrap()
        .get_option_parser()
        .has_option("-v"));
}

#[test]
fn test_inherited_options_dont_affect_parent() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add option to root and mark as inheritable
    app.add_option(
        "verbose",
        "Enable verbose",
        "-v",
        "--verbose",
        ValueTypes::None,
    );
    app.mark_inheritable("-v").unwrap();

    // Parent should not have the subcommand's option (check before creating subcommand)
    assert!(!app.root_command.get_option_parser().has_option("-r"));

    // Create subcommand
    let subcmd = app.command("build", "Build project").unwrap();

    // Add option only to subcommand
    subcmd.add_option(
        "release",
        "Release build",
        "-r",
        "--release",
        ValueTypes::None,
    );

    // Subcommand should have both its own and inherited options
    assert!(subcmd.get_option_parser().has_option("-v"));
    assert!(subcmd.get_option_parser().has_option("-r"));
}

#[test]
fn test_mark_inheritable_using_long_flag() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add option
    app.add_option(
        "verbose",
        "Enable verbose",
        "-v",
        "--verbose",
        ValueTypes::None,
    );

    // Mark using long flag
    let result = app.mark_inheritable("--verbose");
    assert!(result.is_ok());

    // Subcommand should inherit it
    let cmd = app.command("build", "Build project").unwrap();
    assert!(cmd.get_option_parser().has_option("-v"));
    assert!(cmd.get_option_parser().has_option("--verbose"));
}

#[test]
fn test_inheritable_options_with_values() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add option with a default value
    app.add_option(
        "config",
        "Config file",
        "-c",
        "--config",
        ValueTypes::OptionalSingle(Some(Value::Str("config.toml".to_string()))),
    );

    // Mark as inheritable
    app.mark_inheritable("-c").unwrap();

    // Create subcommand
    let cmd = app.command("serve", "Start server").unwrap();

    // Should have the config option with default value
    assert!(cmd.get_option_parser().has_option("-c"));
    assert!(cmd.get_option_parser().has_option("--config"));
}

#[test]
fn test_subcommand_can_override_inherited_option() {
    let mut app = Fli::new("cli", "1.0.0", "CLI app");

    // Add option to root
    app.add_option(
        "port",
        "Port number",
        "-p",
        "--port",
        ValueTypes::OptionalSingle(Some(Value::Int(8080))),
    );
    app.mark_inheritable("-p").unwrap();

    // Create subcommand - it inherits -p
    let cmd = app.command("serve", "Start server").unwrap();

    // Subcommand can still add its own options
    cmd.add_option(
        "host",
        "Host address",
        "-h",
        "--host",
        ValueTypes::OptionalSingle(Some(Value::Str("localhost".to_string()))),
    );

    // Should have both inherited and own options
    assert!(cmd.get_option_parser().has_option("-p"));
    assert!(cmd.get_option_parser().has_option("-h"));
}
