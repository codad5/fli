// Test the init_fli_from_toml macro

#[test]
fn test_init_fli_from_toml_macro() {
    let app = crate::init_fli_from_toml!();

    // Should pull values from Cargo.toml
    assert_eq!(app.name, env!("CARGO_PKG_NAME"));
    assert_eq!(app.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(app.description, env!("CARGO_PKG_DESCRIPTION"));
}

#[test]
fn test_init_fli_from_toml_name() {
    let app = crate::init_fli_from_toml!();
    assert!(!app.name.is_empty());
}

#[test]
fn test_init_fli_from_toml_version() {
    let app = crate::init_fli_from_toml!();
    assert!(!app.version.is_empty());
}

#[test]
fn test_init_fli_from_toml_description() {
    let app = crate::init_fli_from_toml!();
    // Description might be empty, so just check it exists
    let _ = app.description;
}

#[test]
fn test_init_fli_from_toml_returns_fli() {
    let app = crate::init_fli_from_toml!();

    // Should be able to use Fli methods
    assert_eq!(app.root_command.get_name(), "");
}

#[test]
fn test_init_fli_from_toml_multiple_calls() {
    let app1 = crate::init_fli_from_toml!();
    let app2 = crate::init_fli_from_toml!();

    assert_eq!(app1.name, app2.name);
    assert_eq!(app1.version, app2.version);
}

#[test]
fn test_init_fli_from_toml_can_add_commands() {
    let mut app = crate::init_fli_from_toml!();

    let result = app.command("test", "Test command");
    assert!(result.is_ok());
}

#[test]
fn test_init_fli_from_toml_can_add_options() {
    use crate::option_parser::{Value, ValueTypes};

    let mut app = crate::init_fli_from_toml!();
    app.add_option(
        "test",
        "Test option",
        "-t",
        "--test",
        ValueTypes::OptionalSingle(Some(Value::Bool(false))),
    );

    assert!(app.root_command.get_option_parser().has_option("-t"));
}
