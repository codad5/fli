# FileManager CLI - Fli Library Demonstration

A comprehensive command-line file system management tool that demonstrates all the powerful features of the **Fli** CLI library.

## 🎯 Overview

This sample application showcases how to build a full-featured CLI tool using Fli. It implements common file system operations similar to Unix commands (ls, mkdir, rm, cp, mv, etc.) while demonstrating:

- ✅ Command hierarchies and subcommands
- ✅ Multiple value types (String, Integer, Boolean, Float)
- ✅ Required and optional options
- ✅ Single and multiple value arguments
- ✅ Positional arguments
- ✅ Flag options (boolean switches)
- ✅ Callback data access
- ✅ Error handling
- ✅ Colored output
- ✅ Auto-generated help system

## 🚀 Quick Start

### Build and Run

```bash
# Build the application
cargo build --release

# Run with help to see all commands
cargo run -- --help

# Or use the compiled binary
./target/release/sample --help
```

## 📚 Available Commands

### 1. **ls** - List Directory Contents

List files and directories with various formatting options.

```bash
# Basic listing
sample ls

# List with details (long format)
sample ls -l

# Show hidden files
sample ls -a

# Human-readable file sizes
sample ls -lh

# Sort by size
sample ls -l --sort=size

# Combine options
sample ls -alh --sort=time /path/to/directory
```

**Options:**

- `-a, --all` - Show hidden files (starting with .)
- `-l, --long` - Use long listing format with details
- `-h, --human-readable` - Print human-readable sizes (e.g., 1K, 234M)
- `-s, --sort <TYPE>` - Sort by: name, size, time, or extension

**Example Output:**

```
FILE       1.2 KB  README.md
DIR         4.0 KB  src/
FILE      512 B   Cargo.toml
```

### 2. **mkdir** - Create Directories

Create one or more directories with optional parent creation.

```bash
# Create a single directory
sample mkdir mydir

# Create nested directories
sample mkdir -p path/to/nested/directory

# Create multiple directories
sample mkdir dir1 dir2 dir3

# With verbose output
sample -v mkdir -p deep/nested/path
```

**Options:**

- `-p, --parents` - Create parent directories as needed
- `-m, --mode <MODE>` - Set permission mode (default: 755)

### 3. **rm** - Remove Files and Directories

Remove files or directories with safety options.

```bash
# Remove a file
sample rm file.txt

# Remove a directory (requires -r)
sample rm -r directory

# Force removal without prompts
sample rm -rf directory

# Interactive mode (prompt before each removal)
sample rm -i file1.txt file2.txt

# Remove multiple files
sample rm file1.txt file2.txt file3.txt
```

**Options:**

- `-r, --recursive` - Remove directories and their contents recursively
- `-f, --force` - Ignore nonexistent files, never prompt
- `-i, --interactive` - Prompt before every removal

**Safety Features:**

- Prompts before removing (in interactive mode)
- Won't remove directories without `-r` flag
- Clear error messages for missing files

### 4. **cp** - Copy Files and Directories

Copy files or directories with preservation options.

```bash
# Copy a file
sample cp source.txt destination.txt

# Copy to a directory
sample cp file.txt target_directory/

# Copy a directory recursively
sample cp -r source_dir target_dir

# Force overwrite
sample cp -f source.txt existing.txt

# Copy multiple files to a directory
sample cp file1.txt file2.txt file3.txt target_dir/

# With verbose output
sample -v cp -r project/ backup/
```

**Options:**

- `-r, --recursive` - Copy directories recursively
- `-f, --force` - Overwrite existing files without prompting
- `-p, --preserve` - Preserve file attributes (timestamps, permissions)

### 5. **mv** - Move or Rename Files

Move or rename files and directories.

```bash
# Rename a file
sample mv oldname.txt newname.txt

# Move file to directory
sample mv file.txt target_directory/

# Move with force overwrite
sample mv -f source.txt destination.txt

# Interactive mode (prompt before overwrite)
sample mv -i file.txt existing_file.txt

# No clobber (don't overwrite existing files)
sample mv -n source.txt destination.txt
```

**Options:**

- `-f, --force` - Overwrite existing files without prompting
- `-i, --interactive` - Prompt before overwriting
- `-n, --no-clobber` - Don't overwrite existing files

### 6. **cat** - Display File Contents

Display the contents of one or more files.

```bash
# Display a file
sample cat file.txt

# Display multiple files
sample cat file1.txt file2.txt

# Number all output lines
sample cat -n file.txt

# Show line endings
sample cat -E file.txt

# Combine options
sample cat -nE file.txt
```

**Options:**

- `-n, --number` - Number all output lines
- `-E, --show-ends` - Display $ at end of each line

**Example Output:**

```
     1 First line
     2 Second line
     3 Third line
```

### 7. **find** - Search for Files

Search for files and directories matching criteria.

```bash
# Find all files in current directory
sample find

# Find in specific directory
sample find /path/to/search

# Search by name pattern
sample find --name "*.txt"

# Find only directories
sample find --type d

# Find only files
sample find --type f

# Limit search depth
sample find --max-depth 2

# Combine options
sample find . --name "*.rs" --type f
```

**Options:**

- `-n, --name <PATTERN>` - Search by filename pattern (supports wildcards)
- `-t, --type <TYPE>` - Filter by type: f (file), d (directory)
- `-s, --size <SIZE>` - Filter by size in bytes
- `-d, --max-depth <DEPTH>` - Maximum directory depth to search

**Pattern Examples:**

- `*.txt` - All .txt files
- `test*` - Files starting with "test"
- `*config*` - Files containing "config"

### 8. **stat** - Display File Statistics

Show detailed information about files or directories.

```bash
# Display file info
sample stat file.txt

# Display directory info
sample stat mydir/

# JSON output format
sample stat --format json file.txt

# Multiple files
sample stat file1.txt file2.txt directory/
```

**Options:**

- `-f, --format <FORMAT>` - Output format: text or json (default: text)

**Example Output (text):**

```
════════════════════════════════════════════════════════════
File: sample.txt
────────────────────────────────────────────────────────────
Type: File
Size: 1234 bytes (1.21 KB)
Read-only: No
Modified: SystemTime { ... }
Accessed: SystemTime { ... }
Created: SystemTime { ... }
════════════════════════════════════════════════════════════
```

**Example Output (json):**

```json
{
  "path": "sample.txt",
  "type": "file",
  "size": 1234,
  "readonly": false
}
```

### 9. **tree** - Display Directory Tree

Display a visual tree structure of directories.

```bash
# Display tree of current directory
sample tree

# Display tree of specific directory
sample tree /path/to/directory

# Show hidden files
sample tree -a

# Show only directories
sample tree -d

# Limit depth
sample tree --level 2

# Combine options
sample tree -ad --level 3
```

**Options:**

- `-a, --all` - Show hidden files
- `-d, --dirs-only` - List directories only
- `-L, --level <DEPTH>` - Maximum display depth (default: 3)

**Example Output:**

```
.
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── lib.rs
```

## 🎨 Global Options

These options work with all commands:

### **-v, --verbose**

Enable detailed output for operations.

```bash
sample -v mkdir test_directory
# Output: ✓ Created directory: test_directory

sample -v cp file.txt backup.txt
# Output: ✓ Copied 'file.txt' to 'backup.txt'
```

### **-q, --quiet**

Suppress all non-error output.

```bash
sample -q mkdir test_directory
# No output unless there's an error
```

### **-c, --color**

Enable/disable colored output.

```bash
# Disable colors
sample --color=false ls

# Enable colors (default)
sample --color=true ls
```

### **--help**

Display help information.

```bash
# Help for the app
sample --help

# Help for a specific command
sample ls --help
sample mkdir --help
```

### **--version**

Display version information.

```bash
sample --version
```

### **--debug**

Enable debug mode for development.

```bash
sample --debug ls
```

## 🔧 Code Examples

### Example 1: Basic Command with Flags

```rust
app.command("ls", "List directory contents")
    .unwrap()
    .add_option(
        "all",
        "Show hidden files",
        "-a",
        "--all",
        ValueTypes::None  // Flag option (no value)
    )
    .set_callback(|data| {
        let show_all = data.get_option_value("all").is_some();
        // Implementation...
    });
```

### Example 2: Command with String Option

```rust
app.command("mkdir", "Create directory")
    .unwrap()
    .add_option(
        "mode",
        "Set permission mode",
        "-m",
        "--mode",
        ValueTypes::OptionalSingle(Some(Value::Int(755)))
    )
    .set_callback(|data| {
        let mode = data.get_option_value("mode")
            .and_then(|v| match v {
                ValueTypes::OptionalSingle(Some(Value::Int(n))) => Some(*n),
                _ => None,
            })
            .unwrap_or(755);
        // Implementation...
    });
```

### Example 3: Command with Multiple Values

```rust
app.command("cp", "Copy files")
    .unwrap()
    .set_expected_positional_args(2)  // source(s) and destination
    .set_callback(|data| {
        let args = data.get_arguments();
        let dest = &args[args.len() - 1];
        let sources = &args[..args.len() - 1];

        for source in sources {
            // Copy each source to destination
        }
    });
```

### Example 4: Interactive Prompts

```rust
if interactive && !force {
    print!("Remove '{}'? (y/N): ", filename);
    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if !input.trim().eq_ignore_ascii_case("y") {
        return;  // User cancelled
    }
}
```

## 🎓 Key Concepts Demonstrated

### 1. Value Types

```rust
// Flag (no value)
ValueTypes::None

// Required single value
ValueTypes::RequiredSingle(Value::Str(String::new()))

// Optional single value with default
ValueTypes::OptionalSingle(Some(Value::Int(1)))

// Multiple required values
ValueTypes::RequiredMultiple(vec![], None)

// Multiple optional values
ValueTypes::OptionalMultiple(None, None)
```

### 2. Accessing Option Values

```rust
// Check if flag is set
let verbose = data.get_option_value("verbose").is_some();

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

// Get multiple string values
let files = data.get_option_value("files")
    .and_then(|v| v.as_strings())
    .unwrap_or_default();
```

### 3. Positional Arguments

```rust
// Get all positional arguments
let args = data.get_arguments();

// Get specific argument by index
let first_arg = data.get_argument_at(0).unwrap_or("default");

// Set expected count
.set_expected_positional_args(2)  // Requires exactly 2 args
```

### 4. Error Handling

```rust
match fs::read_dir(path) {
    Ok(entries) => {
        // Process entries
    }
    Err(e) => {
        eprintln!("{} Failed to read directory: {}", "✗".red(), e);
    }
}
```

## 🎨 Color Coding

The application uses colors to enhance readability:

- 🔵 **Blue** - Directories
- 🟢 **Green** - Success messages, files
- 🔴 **Red** - Error messages
- 🟡 **Yellow** - Important values, highlighted text
- 🔷 **Cyan** - Informational messages, section separators

## 🛠️ Building Features

### Adding a New Command

1. Create the command structure:

```rust
app.command("mycommand", "Description of command")
    .unwrap()
```

2. Add options:

```rust
    .add_option(
        "option_name",
        "Option description",
        "-s",
        "--long-flag",
        ValueTypes::None  // or other type
    )
```

3. Set callback:

```rust
    .set_callback(|data| {
        // Access options and arguments
        let value = data.get_option_value("option_name");
        let args = data.get_arguments();

        // Implement functionality
    });
```

## 📖 Best Practices

1. **Always provide help text** - Use descriptive command and option descriptions
2. **Validate inputs** - Check arguments before processing
3. **Handle errors gracefully** - Use Result types and match statements
4. **Provide feedback** - Use verbose mode for detailed operations
5. **Use appropriate value types** - Match the type to your data needs
6. **Test edge cases** - Check for missing files, invalid inputs, etc.

## 🐛 Debugging

Enable debug mode to see detailed parsing information:

```bash
sample --debug ls -la
```

This will show:

- How arguments are parsed
- Option values detected
- Command chain execution

## 📝 License

This sample application is part of the Fli library and follows the same MIT license.

## 🤝 Contributing

This is a demonstration application. For contributions to the Fli library itself, please refer to the main repository.

## 📚 Learn More

- [Fli Documentation](https://docs.rs/fli)
- [Fli Repository](https://github.com/codad5/fli)
- [Cargo.toml Reference](https://doc.rust-lang.org/cargo/reference/manifest.html)

---

**Built with ❤️ using the Fli CLI Library**
