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

    let result = app
        .command("build", "Build project")
        .and_then(|cmd| {
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
