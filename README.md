# FLI

A powerful, type-safe CLI library for Rust inspired by [commander.js](https://github.com/tj/commander.js), designed to make building command-line applications intuitive and ergonomic.

[![Crates.io](https://img.shields.io/crates/v/fli.svg)](https://crates.io/crates/fli)
[![Documentation](https://docs.rs/fli/badge.svg)](https://docs.rs/fli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **⚠️ BREAKING CHANGE in v1.2.0**: `ValueTypes::None` has been removed! Use `ValueTypes::OptionalSingle(Some(Value::Bool(false)))` for flag options instead. See the [Migration Guide](#migration-from-valuetypesnone) for details.

> **Note**: Version 1.0.0 included breaking changes. See the [Migration Guide](#migration-from-0x) for upgrading from v0.x.

## Features

- 🎯 **Type-safe value parsing** with compile-time guarantees
- 🌲 **Hierarchical commands** with subcommand support
- 🎨 **Beautiful help output** with automatic formatting
- 🔧 **Flexible option types**: flags, required/optional values, single/multiple values
- 🚀 **Zero-cost abstractions** with minimal overhead
- 📦 **Cargo.toml integration** for automatic metadata
- ⚡ **Error handling** with detailed, actionable messages

## Quick Start

```toml
[dependencies]
fli = "1.0"
```

```rust
use fli::{Fli, ValueTypes, Value};

fn main() {
    let mut app = Fli::new("myapp", "1.0.0", "A sample CLI app");
    
    // Add a simple flag
    app.add_option(
        "verbose",
        "Enable verbose output",
        "-v",
        "--verbose",
        ValueTypes::OptionalSingle(Some(Value::Bool(false)))
    );
    
    // Add an option that requires a value
    app.add_option(
        "name",
        "Your name",
        "-n",
        "--name",
        ValueTypes::RequiredSingle(Value::Str(String::new()))
    );
    
    // Set the callback
    app.set_callback(|data| {
        let name = data.get_option_value("name")
            .and_then(|v| v.as_str())
            .unwrap_or("World");
        
        let verbose = data.get_option_value("verbose").is_some();
        
        println!("Hello, {}!", name);
        if verbose {
            println!("Verbose mode enabled!");
        }
    });
    
    app.run();
}
```

Run it:
```bash
$ cargo run -- -n Alice -v
Hello, Alice!
Verbose mode enabled!
```

## Core Concepts

### Value Types

Fli provides explicit value types for type-safe option parsing:

```rust
use fli::{ValueTypes, Value};

// Flag option (no value)
ValueTypes::OptionalSingle(Some(Value::Bool(false)))

// Required single value
ValueTypes::RequiredSingle(Value::Str(String::new()))
ValueTypes::RequiredSingle(Value::Int(0))

// Optional single value with default
ValueTypes::OptionalSingle(Some(Value::Int(8080)))
ValueTypes::OptionalSingle(None)

// Required multiple values (at least 1)
ValueTypes::RequiredMultiple(vec![], None)

// Exactly N values required
ValueTypes::RequiredMultiple(vec![], Some(3))

// Optional multiple values
ValueTypes::OptionalMultiple(None, None)

// Up to N values
ValueTypes::OptionalMultiple(None, Some(5))
```

### Commands and Subcommands

Create hierarchical command structures:

```rust
use fli::{Fli, ValueTypes, Value};

fn main() {
    let mut app = Fli::new("git", "2.0.0", "Version control system");
    
    // Create a command
    let commit_cmd = app.command("commit", "Record changes").unwrap();
    
    // Add options to the command
    commit_cmd.add_option(
        "message",
        "Commit message",
        "-m",
        "--message",
        ValueTypes::RequiredSingle(Value::Str(String::new()))
    );
    
    commit_cmd.add_option(
        "all",
        "Commit all changes",
        "-a",
        "--all",
        ValueTypes::OptionalSingle(Some(Value::Bool(false)))
    );
    
    // Set the command callback
    commit_cmd.set_callback(|data| {
        let message = data.get_option_value("message")
            .and_then(|v| v.as_str())
            .unwrap_or("No message");
        
        let all = data.get_option_value("all").is_some();
        
        println!("Committing with message: {}", message);
        if all {
            println!("Including all changes");
        }
    });
    
    app.run();
}
```

### Accessing Values in Callbacks

The `FliCallbackData` provides convenient methods for accessing parsed values:

```rust
use fli::option_parser::{Value, ValueTypes};

fn my_callback(data: &FliCallbackData) {
    // Get a single string value
    let name = data.get_option_value("name")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    
    // Get multiple string values
    let files = data.get_option_value("files")
        .and_then(|v| v.as_strings())
        .unwrap_or_default();
    
    // Check if a flag was passed (NEW in v1.2.0)
    // Flags use Bool values: false = not passed, true = passed
    let verbose = data.get_option_value("verbose")
        .map(|v| matches!(v, ValueTypes::OptionalSingle(Some(Value::Bool(true)))))
        .unwrap_or(false);
    
    // Shorter alternative for flags:
    let is_flag_set = data.get_option_value("verbose").is_some();
    
    // Get positional arguments
    let first_arg = data.get_argument_at(0);
    let all_args = data.get_arguments();
    
    // Access the command
    let cmd_name = data.get_command().get_name();
}
```

## Examples

### Basic Calculator

```rust
use fli::{Fli, ValueTypes, Value};

fn main() {
    let mut app = Fli::new("calc", "1.0.0", "Simple calculator");
    
    let calc_cmd = app.command("calculate", "Perform calculation").unwrap();
    
    calc_cmd.add_option(
        "operation",
        "Operation to perform (add, sub, mul, div)",
        "-o",
        "--operation",
        ValueTypes::RequiredSingle(Value::Str(String::new()))
    );
    
    calc_cmd.set_expected_positional_args(2);
    
    calc_cmd.set_callback(|data| {
        let op = data.get_option_value("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("add");
        
        let args = data.get_arguments();
        if args.len() < 2 {
            eprintln!("Error: Need two numbers");
            return;
        }
        
        let a: f64 = args[0].parse().unwrap_or(0.0);
        let b: f64 = args[1].parse().unwrap_or(0.0);
        
        let result = match op {
            "add" => a + b,
            "sub" => a - b,
            "mul" => a * b,
            "div" => a / b,
            _ => {
                eprintln!("Unknown operation: {}", op);
                return;
            }
        };
        
        println!("{} {} {} = {}", a, op, b, result);
    });
    
    app.run();
}
```

Usage:
```bash
$ calc calculate -o add 10 20
10 add 20 = 30
```

### File Operations with Subcommands

```rust
use fli::{Fli, ValueTypes, Value};

fn main() {
    let mut app = Fli::new("file-tool", "1.0.0", "File operations");
    
    // File command with subcommands
    let file_cmd = app.command("file", "File operations").unwrap();
    
    // Copy subcommand
    file_cmd.subcommand("copy", "Copy files")
        .add_option(
            "source",
            "Source files",
            "-s",
            "--source",
            ValueTypes::RequiredMultiple(vec![], None)
        )
        .add_option(
            "dest",
            "Destination directory",
            "-d",
            "--dest",
            ValueTypes::RequiredSingle(Value::Str(String::new()))
        )
        .set_callback(|data| {
            let sources = data.get_option_value("source")
                .and_then(|v| v.as_strings())
                .unwrap_or_default();
            
            let dest = data.get_option_value("dest")
                .and_then(|v| v.as_str())
                .unwrap_or(".");
            
            println!("Copying {:?} to {}", sources, dest);
        });
    
    // Move subcommand
    file_cmd.subcommand("move", "Move files")
        .add_option(
            "path",
            "Files to move",
            "-p",
            "--path",
            ValueTypes::RequiredMultiple(vec![], None)
        )
        .set_callback(|data| {
            let paths = data.get_option_value("path")
                .and_then(|v| v.as_strings())
                .unwrap_or_default();
            
            println!("Moving: {:?}", paths);
        });
    
    app.run();
}
```

Usage:
```bash
$ file-tool file copy -s file1.txt file2.txt -d /backup
Copying ["file1.txt", "file2.txt"] to /backup

$ file-tool file move -p old1.txt old2.txt
Moving: ["old1.txt", "old2.txt"]
```

### Greeting App with Options

```rust
use fli::{Fli, ValueTypes, Value};

fn main() {
    let mut app = Fli::new("greet", "1.0.0", "Greeting application");
    
    let greet_cmd = app.command("greet", "Greet someone").unwrap();
    
    greet_cmd.add_option(
        "name",
        "Name to greet",
        "-n",
        "--name",
        ValueTypes::RequiredSingle(Value::Str(String::new()))
    );
    
    greet_cmd.add_option(
        "time",
        "Time of day (morning, afternoon, evening)",
        "-t",
        "--time",
        ValueTypes::OptionalSingle(None)
    );
    
    greet_cmd.add_option(
        "repeat",
        "Number of times to repeat",
        "-r",
        "--repeat",
        ValueTypes::OptionalSingle(Some(Value::Int(1)))
    );
    
    greet_cmd.set_callback(|data| {
        let name = data.get_option_value("name")
            .and_then(|v| v.as_str())
            .unwrap_or("friend");
        
        let time = data.get_option_value("time")
            .and_then(|v| v.as_str())
            .unwrap_or("Hello");
        
        let greeting = match time {
            "morning" => "Good morning",
            "afternoon" => "Good afternoon",
            "evening" => "Good evening",
            _ => "Hello",
        };
        
        let repeat = data.get_option_value("repeat")
            .and_then(|v| match v {
                ValueTypes::OptionalSingle(Some(Value::Int(n))) => Some(*n),
                _ => None,
            })
            .unwrap_or(1);
        
        for _ in 0..repeat {
            println!("{}, {}!", greeting, name);
        }
    });
    
    app.run();
}
```

Usage:
```bash
$ greet greet -n Alice -t morning -r 2
Good morning, Alice!
Good morning, Alice!
```

## Advanced Features

### Custom Help Messages

Fli automatically generates beautiful help messages:

```bash
$ myapp --help
Command: myapp

A sample CLI application

Usage
  myapp [SUBCOMMANDS] [ARGUMENT] [ARGUMENT] [OPTIONS]
  [SUBCOMMANDS] [OPTIONS]  -- [ARGUMENT] [ARGUMENT]

Options
┌───────┬────────────┬──────────────────────┬─────────────────────┐
│ Flag  │ Long Form  │ Value Type           │ Description         │
├───────┼────────────┼──────────────────────┼─────────────────────┤
│ -v    │ --verbose  │ none                 │ Enable verbose      │
│ -n    │ --name     │ single (required)    │ Your name           │
└───────┴────────────┴──────────────────────┴─────────────────────┘
```

### Debug Mode

Enable debug output to see internal parsing details:

```rust
let app = Fli::new("myapp", "1.0.0", "Description")
    .with_debug();

// Or add a debug flag
app.add_debug_option();
```

### Error Handling

Commands return `Result<&mut FliCommand>` for better error handling:

```rust
use fli::Result;

fn main() -> Result<()> {
    let mut app = Fli::new("myapp", "1.0.0", "Description");
    
    let cmd = app.command("serve", "Start server")?;
    cmd.add_option(
        "port",
        "Port to bind",
        "-p",
        "--port",
        ValueTypes::RequiredSingle(Value::Int(8080))
    );
    
    app.run();
    Ok(())
}
```

### Cargo.toml Integration

Initialize from your `Cargo.toml` metadata:

```rust
use fli::init_fli_from_toml;

fn main() {
    let mut app = init_fli_from_toml!();
    
    // App name, version, and description are automatically loaded
    // from your Cargo.toml file
    
    app.run();
}
```

## Migration from 0.x

Version 1.0.0 includes significant improvements but breaks backwards compatibility. Key changes:

### Option Syntax

**Before (v0.x):**
```rust
app.option("-n --name, <>", "Your name", callback);
```

**After (v1.0):**
```rust
app.add_option(
    "name",
    "Your name",
    "-n",
    "--name",
    ValueTypes::RequiredSingle(Value::Str(String::new()))
);
app.set_callback(callback);
```

### Value Types

| v0.x | v1.0 |
|------|------|
| (no symbol) | `ValueTypes::OptionalSingle(Some(Value::Bool(false)))` |
| `<>` | `ValueTypes::RequiredSingle(_)` |
| `[]` | `ValueTypes::OptionalSingle(_)` |
| `<...>` | `ValueTypes::RequiredMultiple(vec![], None)` |
| `[...]` | `ValueTypes::OptionalMultiple(None, None)` |

### Callbacks

**Before (v0.x):**
```rust
fn callback(app: &Fli) {
    let name = app.get_values("name".to_owned()).unwrap()[0];
}
```

**After (v1.0):**
```rust
fn callback(data: &FliCallbackData) {
    let name = data.get_option_value("name")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
}
```

### Commands

**Before (v0.x):**
```rust
let cmd = app.command("serve", "Start server");
```

**After (v1.0):**
```rust
let cmd = app.command("serve", "Start server")?; // Returns Result
```

For a complete migration guide, see [MIGRATION.md](MIGRATION.md).

## Migration from ValueTypes::None

**⚠️ Breaking Change in v1.2.0**

`ValueTypes::None` has been removed to fix a critical design flaw where you couldn't distinguish between "flag was defined" and "flag was passed".

### Quick Fix

**Before (v1.1 and earlier):**
```rust
app.add_option(
    "verbose",
    "Enable verbose output",
    "-v",
    "--verbose",
    ValueTypes::None  // ❌ Removed in v1.2.0
);
```

**After (v1.2.0+):**
```rust
use fli::option_parser::{Value, ValueTypes};

app.add_option(
    "verbose",
    "Enable verbose output",
    "-v",
    "--verbose",
    ValueTypes::OptionalSingle(Some(Value::Bool(false)))  // ✅ Use Bool(false) as default
);
```

### Checking if a Flag was Passed

**Before (v1.1 - didn't work correctly):**
```rust
// This would always be true if the option was defined!
let verbose = data.get_option_value("verbose").is_some();
```

**After (v1.2.0 - works correctly):**
```rust
// Method 1: Check the boolean value
let verbose = data.get_option_value("verbose")
    .map(|v| matches!(v, ValueTypes::OptionalSingle(Some(Value::Bool(true)))))
    .unwrap_or(false);

// Method 2: Simpler (still works, checks if value was updated)
let verbose = data.get_option_value("verbose").is_some();
```

### Why the Change?

With `ValueTypes::None`:
- ❌ Could not tell if `-v` was passed or not
- ❌ `get_option_value()` always returned `Some` for defined options
- ❌ Had to search command chain to check flag usage

With `ValueTypes::OptionalSingle(Some(Value::Bool(...)))`:
- ✅ `Bool(false)` = flag not passed (default)
- ✅ `Bool(true)` = flag was passed
- ✅ Can query option parser directly
- ✅ Clear, idiomatic Rust

## Documentation

- [API Documentation](https://docs.rs/fli)
- [Examples](https://github.com/codad5/fli/tree/master/examples)
- [Changelog](CHANGELOG.md)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [commander.js](https://github.com/tj/commander.js)
- Built with ❤️ by [Chibueze Aniezeofor (codad5)](https://github.com/codad5)