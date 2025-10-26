# Developer Guide - FileManager CLI Sample

This guide explains how the FileManager CLI sample application is structured and how to extend it with new features.

## Architecture Overview

The application demonstrates a well-structured CLI application using the Fli library:

```
sample/
├── src/
│   └── main.rs          # Main application with all commands
├── Cargo.toml           # Dependencies and metadata
├── README.md            # User documentation
├── EXAMPLES.md          # Usage examples
└── DEVELOPER.md         # This file
```

## Code Structure

### 1. Application Initialization

```rust
fn main() {
    // Initialize from Cargo.toml with debug mode
    let mut app = init_fli_from_toml!()
        .with_debug()
        .add_debug_option();

    // Add global options
    app.add_option(...);

    // Add commands
    app.command(...).set_callback(...);

    // Run the application
    app.run();
}
```

### 2. Command Structure

Each command follows this pattern:

```rust
app.command("command_name", "Description")
    .unwrap()
    .add_option("opt_name", "Description", "-s", "--long", ValueType)
    .set_expected_positional_args(count)
    .set_callback(|data| {
        // Command implementation
    });
```

### 3. Value Types

The application uses all available value types:

#### Flag (No Value)

```rust
ValueTypes::None
// Example: -v, --verbose flags
```

#### Required Single Value

```rust
ValueTypes::RequiredSingle(Value::Str(String::new()))
// Example: --name <NAME> (required)
```

#### Optional Single Value with Default

```rust
ValueTypes::OptionalSingle(Some(Value::Int(755)))
// Example: --mode <MODE> (default: 755)
```

#### Optional Single Value without Default

```rust
ValueTypes::OptionalSingle(None)
// Example: --config <FILE> (optional, no default)
```

#### Multiple Required Values

```rust
ValueTypes::RequiredMultiple(vec![], None)
// Example: source files for copy command
```

#### Multiple Optional Values

```rust
ValueTypes::OptionalMultiple(None, None)
// Example: optional list of tags
```

## Adding a New Command

### Step-by-Step Guide

1. **Plan the Command**

   - What should it do?
   - What options does it need?
   - What arguments should it accept?

2. **Add Command Structure**

```rust
app.command("mycommand", "Do something useful")
    .unwrap()
```

3. **Add Options**

```rust
    .add_option(
        "option_name",          // Internal name
        "Option description",   // Help text
        "-s",                   // Short flag
        "--long-option",        // Long flag
        ValueTypes::None        // Value type
    )
```

4. **Set Positional Arguments (if needed)**

```rust
    .set_expected_positional_args(1)  // Number of required args
```

5. **Implement Callback**

```rust
    .set_callback(|data| {
        // Get option values
        let verbose = data.get_option_value("verbose").is_some();

        // Get positional arguments
        let args = data.get_arguments();

        // Implement functionality
        println!("Doing something...");
    });
```

### Complete Example: Adding a "touch" Command

```rust
// Create files with current timestamp
app.command("touch", "Create empty files or update timestamps")
    .unwrap()
    .add_option(
        "no-create",
        "Don't create files if they don't exist",
        "-c",
        "--no-create",
        ValueTypes::None
    )
    .add_option(
        "timestamp",
        "Use specific timestamp",
        "-t",
        "--timestamp",
        ValueTypes::OptionalSingle(None)
    )
    .set_expected_positional_args(1)
    .set_callback(|data| {
        let files = data.get_arguments();
        let no_create = data.get_option_value("no-create").is_some();
        let verbose = data.get_option_value("verbose").is_some();

        if files.is_empty() {
            eprintln!("{} No file specified", "✗".red());
            return;
        }

        for file in files {
            let path = Path::new(&file);

            if path.exists() {
                // Update timestamp
                if let Err(e) = update_file_time(path) {
                    eprintln!("{} Failed to update '{}': {}", "✗".red(), file, e);
                } else if verbose {
                    println!("{} Updated timestamp: {}", "✓".green(), file.yellow());
                }
            } else if !no_create {
                // Create new file
                match std::fs::File::create(path) {
                    Ok(_) => {
                        if verbose {
                            println!("{} Created file: {}", "✓".green(), file.yellow());
                        }
                    }
                    Err(e) => eprintln!("{} Failed to create '{}': {}", "✗".red(), file, e),
                }
            }
        }
    });

fn update_file_time(path: &Path) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    OpenOptions::new().write(true).open(path)?;
    Ok(())
}
```

## Working with Callback Data

### Accessing Option Values

```rust
.set_callback(|data| {
    // Check if flag is set
    let is_verbose = data.get_option_value("verbose").is_some();

    // Get string value
    let name = data.get_option_value("name")
        .and_then(|v| v.as_str())
        .unwrap_or("default");

    // Get integer value
    let count = data.get_option_value("count")
        .and_then(|v| match v {
            ValueTypes::OptionalSingle(Some(Value::Int(n))) => Some(*n),
            _ => None,
        })
        .unwrap_or(1);

    // Get float value
    let threshold = data.get_option_value("threshold")
        .and_then(|v| match v {
            ValueTypes::OptionalSingle(Some(Value::Float(f))) => Some(*f),
            _ => None,
        })
        .unwrap_or(0.5);

    // Get boolean value
    let enabled = data.get_option_value("enabled")
        .and_then(|v| match v {
            ValueTypes::OptionalSingle(Some(Value::Bool(b))) => Some(*b),
            _ => None,
        })
        .unwrap_or(false);

    // Get multiple string values
    let files = data.get_option_value("files")
        .and_then(|v| v.as_strings())
        .unwrap_or_default();
});
```

### Accessing Positional Arguments

```rust
.set_callback(|data| {
    // Get all arguments as Vec<String>
    let args = data.get_arguments();

    // Get specific argument by index
    let first = data.get_argument_at(0).map(|s| s.as_str()).unwrap_or(".");
    let second = data.get_argument_at(1).map(|s| s.as_str());

    // Check number of arguments
    if args.len() < 2 {
        eprintln!("Error: Expected at least 2 arguments");
        return;
    }

    // Split arguments (e.g., for cp SOURCE... DEST)
    let dest = &args[args.len() - 1];
    let sources = &args[..args.len() - 1];
});
```

## Helper Functions

### format_size - Human-readable file sizes

```rust
fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[0])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

// Usage:
println!("Size: {}", format_size(1536));  // "1.50 KB"
println!("Size: {}", format_size(2097152));  // "2.00 MB"
```

### copy_dir_recursive - Recursive directory copy

```rust
fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
```

### search_files - Pattern matching file search

```rust
fn search_files(
    path: &Path,
    pattern: &str,
    file_type: Option<&str>,
    depth: usize,
    max_depth: usize
) {
    if depth > max_depth {
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_path = entry.path();
            let is_dir = entry_path.is_dir();

            // Apply filters and matching logic
            // ...

            if is_dir {
                search_files(&entry_path, pattern, file_type, depth + 1, max_depth);
            }
        }
    }
}
```

### display_tree - Directory tree visualization

```rust
fn display_tree(
    path: &Path,
    prefix: &str,
    show_all: bool,
    dirs_only: bool,
    level: usize,
    max_level: usize
) {
    if level >= max_level {
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        let mut items: Vec<_> = entries.filter_map(Result::ok).collect();
        items.sort_by_key(|e| e.file_name());

        let count = items.len();

        for (i, entry) in items.iter().enumerate() {
            let is_last = i == count - 1;
            let connector = if is_last { "└── " } else { "├── " };
            let extension = if is_last { "    " } else { "│   " };

            // Display logic...
        }
    }
}
```

## Color Usage

Using the `colored` crate for enhanced output:

```rust
use colored::Colorize;

// Status indicators
println!("{} Success message", "✓".green());
println!("{} Error message", "✗".red());
println!("{} Info message", "→".cyan());
println!("{} Warning message", "⚠".yellow());

// Colored text
println!("Directory: {}", path.blue());
println!("File: {}", filename.green());
println!("Value: {}", value.yellow());
println!("Header: {}", title.bold());

// Colored backgrounds
println!("{}", "CRITICAL".red().bold().on_white());

// Combinations
println!("{} {} {}",
    "Status:".bold(),
    "Running".green(),
    "(active)".cyan()
);
```

## Error Handling Best Practices

### 1. Use Result Types

```rust
fn process_file(path: &Path) -> std::io::Result<()> {
    let content = fs::read_to_string(path)?;
    // Process content
    Ok(())
}

// In callback:
match process_file(Path::new(&file)) {
    Ok(_) => println!("{} Processed successfully", "✓".green()),
    Err(e) => eprintln!("{} Failed: {}", "✗".red(), e),
}
```

### 2. Provide Helpful Error Messages

```rust
// Bad
eprintln!("Error");

// Good
eprintln!("{} Failed to read directory '{}': {}",
    "✗".red(),
    path,
    e
);

// Better
eprintln!("{} Failed to read directory '{}'", "✗".red(), path);
eprintln!("  Reason: {}", e);
eprintln!("  Hint: Check if the directory exists and you have permission");
```

### 3. Handle Edge Cases

```rust
// Check for empty input
if files.is_empty() {
    eprintln!("{} No files specified", "✗".red());
    return;
}

// Check for existence
if !path.exists() {
    eprintln!("{} Path does not exist: {}", "✗".red(), path);
    return;
}

// Check permissions
if metadata.permissions().readonly() && operation_needs_write {
    eprintln!("{} Cannot write to read-only file", "✗".red());
    return;
}
```

## Testing

### Manual Testing

```bash
# Test each command
cargo run -- ls --help
cargo run -- mkdir test_dir
cargo run -- ls -l
cargo run -- tree
cargo run -- rm -r test_dir
```

### Integration Testing

Create a test script `test_all.sh`:

```bash
#!/bin/bash
set -e

echo "Building..."
cargo build --release

BIN="./target/release/sample"

echo "Testing mkdir..."
$BIN mkdir test_dir
[ -d test_dir ] || exit 1

echo "Testing file creation..."
echo "test" > test_dir/file.txt

echo "Testing ls..."
$BIN ls test_dir | grep "file.txt" || exit 1

echo "Testing cp..."
$BIN cp -r test_dir test_backup
[ -d test_backup ] || exit 1

echo "Testing mv..."
$BIN mv test_dir/file.txt test_dir/renamed.txt
[ -f test_dir/renamed.txt ] || exit 1

echo "Testing rm..."
$BIN rm -rf test_dir test_backup
[ ! -d test_dir ] || exit 1

echo "All tests passed!"
```

## Performance Considerations

### 1. Large Directory Listings

```rust
// Instead of loading all entries into memory:
for entry in fs::read_dir(path)? {
    // Process immediately
    process_entry(entry?);
}

// Rather than:
let entries: Vec<_> = fs::read_dir(path)?.collect();
// This loads everything into memory first
```

### 2. Efficient String Operations

```rust
// Use string slices when possible
fn process(path: &str) { }  // Good

fn process(path: String) { }  // Requires ownership

// Use .as_str() to convert
let owned: String = get_string();
process(owned.as_str());
```

### 3. Minimize System Calls

```rust
// Cache metadata when needed multiple times
let metadata = fs::metadata(path)?;
let size = metadata.len();
let is_dir = metadata.is_dir();
let modified = metadata.modified()?;

// Rather than:
fs::metadata(path)?.len();
fs::metadata(path)?.is_dir();  // Duplicate system call
```

## Debugging

### Enable Debug Output

```bash
# Built-in debug mode
cargo run -- --debug ls

# Rust logging
RUST_LOG=debug cargo run -- ls
```

### Add Debug Prints

```rust
.set_callback(|data| {
    if data.get_option_value("debug").is_some() {
        eprintln!("[DEBUG] Arguments: {:?}", data.get_arguments());
        eprintln!("[DEBUG] Options: {:?}", /* option values */);
    }

    // Command implementation
});
```

## Best Practices Summary

1. ✅ **Always validate inputs** before processing
2. ✅ **Provide clear error messages** with context
3. ✅ **Use verbose mode** for detailed operations
4. ✅ **Implement interactive mode** for destructive operations
5. ✅ **Follow Unix command conventions** for familiar UX
6. ✅ **Use colors judiciously** to enhance readability
7. ✅ **Write descriptive help text** for all commands and options
8. ✅ **Handle edge cases** (empty input, missing files, etc.)
9. ✅ **Use Result types** for proper error propagation
10. ✅ **Test thoroughly** with various input combinations

## Resources

- [Fli Documentation](https://docs.rs/fli)
- [Fli Repository](https://github.com/codad5/fli)
- [Colored Crate](https://docs.rs/colored)
- [Rust std::fs](https://doc.rust-lang.org/std/fs/)
- [Rust std::path](https://doc.rust-lang.org/std/path/)

---

**Happy coding! 🦀**
