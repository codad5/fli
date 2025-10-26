# Migration Guide: v0.1.x to v1.0.0

This document provides a comprehensive comparison between v0.1.x and v1.0.0 of the Fli library.

## Table of Contents
- [Overview](#overview)
- [Breaking Changes](#breaking-changes)
- [API Comparison](#api-comparison)
- [Migration Examples](#migration-examples)

## Overview

Version 1.0.0 is a complete rewrite of Fli with a focus on:
- **Type Safety**: Move from string-based parsing to enum-based type system
- **Error Handling**: Proper `Result` types with detailed error variants
- **Better API**: More intuitive methods and clearer separation of concerns
- **Improved UX**: Beautiful help output with tables and better error messages

## Breaking Changes

### 1. Initialization

**v0.1.x:**
```rust
use fli::{Fli, init_fli_from_toml};

let mut app = Fli::init("app-name", "description");
app.set_version("0.0.1");

// OR
let mut app = init_fli_from_toml!();
```

**v1.0.0:**
```rust
use fli::Fli;

let mut app = Fli::new("app-name", "1.0.0", "description");

// OR
let mut app = fli::init_fli_from_toml!();
```

### 2. Option Definition

**v0.1.x:**
```rust
app.option("-n --name, <>", "Name to call you", |x: &Fli| {
    match x.get_values("name".to_owned()) {
        Ok(value) => println!("Hello {}", value[0]),
        Err(_) => {},
    }
});
```

**v1.0.0:**
```rust
use fli::{ValueTypes, Value};

app.add_option(
    "name",
    "Name to call you",
    "-n",
    "--name",
    ValueTypes::RequiredSingle(Value::Str(String::new()))
);

app.set_callback(|data| {
    let name = data.get_option_value("name")
        .and_then(|v| v.as_str())
        .unwrap_or("World");
    println!("Hello {}", name);
});
```

### 3. Value Type Mapping

| v0.1.x Syntax | Meaning | v1.0.0 Type |
|---------------|---------|-------------|
| (no symbol) | Flag only | `ValueTypes::None` |
| `<>` | Required single | `ValueTypes::RequiredSingle(Value::Str(String::new()))` |
| `[]` | Optional single | `ValueTypes::OptionalSingle(None)` |
| `<...>` | Required multiple (at least 1) | `ValueTypes::RequiredMultiple(vec![], None)` |
| `[...]` | Optional multiple | `ValueTypes::OptionalMultiple(None, None)` |

### 4. Callback Signature

**v0.1.x:**
```rust
fn callback(app: &Fli) {
    // Access app state directly
    let values = app.get_values("name".to_owned()).unwrap();
    let is_passed = app.is_passed("--verbose");
}
```

**v1.0.0:**
```rust
fn callback(data: &FliCallbackData) {
    // Access through data object
    let name = data.get_option_value("name")
        .and_then(|v| v.as_str());
    let verbose = data.get_option_value("verbose").is_some();
}
```

### 5. Commands

**v0.1.x:**
```rust
app.command("greet", "An app that respects")
    .default(greet)
    .allow_inital_no_param_values(false)
    .option("-n --name, <>", "Your name", greet);
```

**v1.0.0:**
```rust
let greet_cmd = app.command("greet", "An app that respects")?;

greet_cmd.add_option(
    "name",
    "Your name",
    "-n",
    "--name",
    ValueTypes::RequiredSingle(Value::Str(String::new()))
);

greet_cmd.set_callback(greet);
greet_cmd.set_expected_positional_args(1);
```

### 6. Value Access Methods

| v0.1.x Method | v1.0.0 Equivalent | Notes |
|---------------|-------------------|-------|
| `app.get_values("name")` | `data.get_option_value("name").and_then(\|v\| v.as_str())` | Type-safe extraction |
| `app.is_passed("--flag")` | `data.get_option_value("flag").is_some()` | Check if option passed |
| `app.has_a_value("-n")` | `data.get_option_value("n").and_then(\|v\| v.as_str()).is_some()` | Check if has value |
| `app.get_arg_at(0)` | `data.get_argument_at(0)` | Positional arguments |

## API Comparison

### Removed Methods (v0.1.x → v1.0.0)

| v0.1.x Method | Reason | v1.0.0 Alternative |
|---------------|--------|-------------------|
| `.allow_duplicate_callback()` | Architecture change | Callbacks managed differently |
| `.allow_inital_no_param_values()` | Replaced | `.set_expected_positional_args()` |
| `.default()` | Renamed | `.set_callback()` |
| `.init()` | Renamed | `.new()` |
| `.set_version()` | Combined with init | Pass version to `.new()` |
| `.get_values()` | On `Fli` | Use `FliCallbackData::get_option_value()` |
| `.is_passed()` | On `Fli` | Use `FliCallbackData::get_option_value().is_some()` |
| `.has_a_value()` | On `Fli` | Check value type with `get_option_value()` |
| `.get_arg_at()` | On `Fli` | Use `FliCallbackData::get_argument_at()` |
| `.print_help()` | Manual | Automatic via `--help` flag |

### New Methods (v1.0.0)

| Method | Description |
|--------|-------------|
| `.add_option()` | Type-safe option definition |
| `.set_callback()` | Set command callback |
| `.with_debug()` | Enable debug mode |
| `.add_debug_option()` | Add `-D/--debug` flag |
| `.set_expected_positional_args()` | Set expected arg count |
| `.get_option_value()` | Get parsed option value |
| `.as_str()` | Extract string from value |
| `.as_strings()` | Extract strings from multiple values |
| `.get_arguments()` | Get all positional args |

## Migration Examples

### Example 1: Simple Flag

**v0.1.x:**
```rust
use fli::Fli;

fn main() {
    let mut app = Fli::init("myapp", "My app");
    
    app.option("-v --verbose", "Enable verbose", |x: &Fli| {
        println!("Verbose mode enabled!");
    });
    
    app.run();
}
```

**v1.0.0:**
```rust
use fli::{Fli, ValueTypes};

fn main() {
    let mut app = Fli::new("myapp", "1.0.0", "My app");
    
    app.add_option(
        "verbose",
        "Enable verbose",
        "-v",
        "--verbose",
        ValueTypes::None
    );
    
    app.set_callback(|data| {
        if data.get_option_value("verbose").is_some() {
            println!("Verbose mode enabled!");
        }
    });
    
    app.run();
}
```

### Example 2: Required Value

**v0.1.x:**
```rust
app.option("-n --name, <>", "Your name", |x: &Fli| {
    match x.get_values("name".to_owned()) {
        Ok(values) => println!("Hello, {}!", values[0]),
        Err(e) => eprintln!("Error: {}", e),
    }
});
```

**v1.0.0:**
```rust
use fli::{ValueTypes, Value};

app.add_option(
    "name",
    "Your name",
    "-n",
    "--name",
    ValueTypes::RequiredSingle(Value::Str(String::new()))
);

app.set_callback(|data| {
    let name = data.get_option_value("name")
        .and_then(|v| v.as_str())
        .unwrap_or("World");
    println!("Hello, {}!", name);
});
```

### Example 3: Commands with Options

**v0.1.x:**
```rust
fn main() {
    let mut app = init_fli_from_toml!();
    
    app.command("greet", "Greeting command")
        .default(greet)
        .option("-n --name, <>", "Your name", greet)
        .option("-t --time, []", "Time of day", greet);
    
    app.run();
}

fn greet(x: &Fli) {
    let name = x.get_values("name".to_owned())
        .unwrap_or(vec!["friend".to_string()])[0].clone();
    println!("Hello, {}!", name);
}
```

**v1.0.0:**
```rust
use fli::{Fli, ValueTypes, Value};

fn main() {
    let mut app = fli::init_fli_from_toml!();
    
    let greet_cmd = app.command("greet", "Greeting command").unwrap();
    
    greet_cmd.add_option(
        "name",
        "Your name",
        "-n",
        "--name",
        ValueTypes::RequiredSingle(Value::Str(String::new()))
    );
    
    greet_cmd.add_option(
        "time",
        "Time of day",
        "-t",
        "--time",
        ValueTypes::OptionalSingle(None)
    );
    
    greet_cmd.set_callback(greet);
    
    app.run();
}

fn greet(data: &FliCallbackData) {
    let name = data.get_option_value("name")
        .and_then(|v| v.as_str())
        .unwrap_or("friend");
    
    let time = data.get_option_value("time")
        .and_then(|v| v.as_str());
    
    let greeting = match time {
        Some("morning") => "Good morning",
        Some("evening") => "Good evening",
        _ => "Hello",
    };
    
    println!("{}, {}!", greeting, name);
}
```

### Example 4: Multiple Values

**v0.1.x:**
```rust
app.command("move", "Move files")
    .option("-p --path, <...>", "Files to move", |x: &Fli| {
        match x.get_values("path".to_owned()) {
            Ok(files) => {
                for file in files {
                    println!("Moving: {}", file);
                }
            },
            Err(_) => {},
        }
    });
```

**v1.0.0:**
```rust
use fli::{ValueTypes, Value};

let move_cmd = app.command("move", "Move files")?;

move_cmd.add_option(
    "path",
    "Files to move",
    "-p",
    "--path",
    ValueTypes::RequiredMultiple(vec![], None)
);

move_cmd.set_callback(|data| {
    let files = data.get_option_value("path")
        .and_then(|v| v.as_strings())
        .unwrap_or_default();
    
    for file in files {
        println!("Moving: {}", file);
    }
});
```

## Benefits of v1.0.0

1. **Type Safety**: Compile-time guarantees about option values
2. **Better Errors**: Detailed error messages with suggestions
3. **Cleaner API**: More intuitive method names and organization
4. **Documentation**: Comprehensive docs with examples
5. **Testing**: 110+ tests ensuring reliability
6. **Help System**: Beautiful auto-generated help with tables
7. **Error Recovery**: Better error handling with `Result` types
8. **Performance**: More efficient parsing with state machine
9. **Maintainability**: Modular codebase easier to extend

## Quick Reference Card

### Common Tasks

| Task | v0.1.x | v1.0.0 |
|------|--------|--------|
| Create app | `Fli::init("name", "desc")` | `Fli::new("name", "ver", "desc")` |
| Add flag | `option("-v --verbose", "...", cb)` | `add_option("verbose", "...", "-v", "--verbose", ValueTypes::None)` |
| Required value | `option("-n --name, <>", "...", cb)` | `add_option("name", "...", "-n", "--name", RequiredSingle(_))` |
| Optional value | `option("-f --file, []", "...", cb)` | `add_option("file", "...", "-f", "--file", OptionalSingle(_))` |
| Multiple values | `option("-f --files, <...>", "...", cb)` | `add_option("files", "...", "-f", "--files", RequiredMultiple(_, None))` |
| Set callback | `.default(callback)` | `.set_callback(callback)` |
| Get value | `app.get_values("name")` | `data.get_option_value("name").as_str()` |
| Check flag | `app.is_passed("--flag")` | `data.get_option_value("flag").is_some()` |
| Positional args | `app.get_arg_at(0)` | `data.get_argument_at(0)` |

## Troubleshooting

### Common Migration Issues

1. **Callback signature errors**: Update from `&Fli` to `&FliCallbackData`
2. **Option syntax errors**: Replace string templates with `add_option()` calls
3. **Value access errors**: Use `get_option_value()` instead of `get_values()`
4. **Command errors**: Handle `Result` from `.command()` method
5. **Import errors**: Update imports to include `ValueTypes` and `Value`

## Need Help?

- Check the [examples directory](https://github.com/codad5/fli/tree/master/examples)
- Read the [API documentation](https://docs.rs/fli)
- Open an issue on [GitHub](https://github.com/codad5/fli/issues)
