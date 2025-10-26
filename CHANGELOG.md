# CHANGELOG

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.1] - 2025-10-26

### Changed

- **Code formatting improvements** in error handling module
  - Reformatted struct field definitions in `FliError` enum for better readability
  - Removed unnecessary blank lines for cleaner code organization
- **Code formatting improvements** in value types module
  - Simplified match expression formatting in `replace_with_expected_value` method
  - Improved code consistency and readability
- **Test module organization**
  - Reordered test module declarations alphabetically in `tests/mod.rs`

## [1.1.0] - 2025-10-26

### Added

- **Value mutation and parsing methods** in `Value` enum
  - `replace_with_expected_value(&mut self, new_value: &str) -> Result<Value>` - Updates a value in place by parsing a string according to the value's type
  - `from_str_with_type(template: &Value, input: &str) -> Result<Value>` - Creates a new value from a string using a template for type inference
- **Enhanced boolean parsing** - Accepts multiple formats: true/false, t/f, 1/0, yes/no, y/n (case-insensitive)
- **ValueParseError variant** in `FliError` enum
  - Provides detailed error messages for type parsing failures
  - Includes the failed value, expected type, and reason for failure
- **PartialEq implementation** for `Value` enum
  - Enables value comparison and equality testing
  - Float comparison uses EPSILON for precision handling
- **Comprehensive test suite** for value types
  - 44 tests covering all value creation, parsing, and conversion scenarios
  - Tests for success and failure cases across all supported types (Str, Int, Float, Bool)
  - Tests for `ValueTypes` helper methods (`expects_value`, `as_str`, `as_strings`)

### Changed

- **Input parsing improvements**
  - Enhanced string value handling in option parser
  - Better default value management for optional single values
  - Improved error propagation using `ValueParseError`

### Internal

- Removed obsolete `tests.rs` file in favor of organized `tests/` directory
- Added `value_types_test.rs` with extensive test coverage

## [1.0.0] - 2025-10-24

### 🎉 Major Release - Breaking Changes

This is a complete rewrite of Fli with significant improvements to type safety, error handling, and API design.

### Added

#### Type System

- **Type-safe value parsing** with explicit `ValueTypes` enum
  - `ValueTypes::None` - Flag options with no values
  - `ValueTypes::RequiredSingle(Value)` - Single required value
  - `ValueTypes::OptionalSingle(Option<Value>)` - Single optional value
  - `ValueTypes::RequiredMultiple(Vec<Value>, Option<usize>)` - Multiple required values with optional count constraint
  - `ValueTypes::OptionalMultiple(Option<Vec<Value>>, Option<usize>)` - Multiple optional values with optional count constraint
- **Value enum** supporting multiple types:
  - `Value::Str(String)` - String values
  - `Value::Int(i64)` - Integer values
  - `Value::Float(f64)` - Floating-point values
  - `Value::Bool(bool)` - Boolean values

#### Command System

- **`FliCommand` struct** - Dedicated command structure with:
  - Hierarchical subcommand support
  - Per-command option parsing
  - Command-specific callbacks
  - Expected positional arguments tracking
- **Preserved options** - Special options (like `--help`) that execute immediately
- **Command chaining** - Fluent API for building complex command structures
- **Subcommand support** via `.subcommand()` method

#### Callback System

- **`FliCallbackData` struct** - Rich context passed to callbacks containing:
  - Parsed options via `get_option_value()`
  - Positional arguments via `get_arguments()` and `get_argument_at()`
  - Reference to the executing command
  - Access to the argument parser
- **Convenient value extraction methods**:
  - `.as_str()` - Extract string from single value
  - `.as_strings()` - Extract strings from multiple values
  - Support for multiple lookup formats (with/without dashes)

#### Parser

- **`InputArgsParser`** - Sophisticated argument parser with:
  - State machine-based parsing
  - Support for `--` separator for positional arguments
  - Proper handling of option-value relationships
  - Validation of required values
- **`CommandChain` enum** - Structured representation of parsed arguments:
  - `SubCommand(String)` - Subcommand invocations
  - `Option(String, ValueTypes)` - Options with their values
  - `Argument(String)` - Positional arguments
  - `IsPreservedOption(String)` - Preserved options for immediate execution

#### Error Handling

- **`FliError` enum** with detailed error types:
  - `UnknownCommand` - Unknown command with suggestions
  - `OptionNotFound` - Option lookup failures
  - `MissingValue` - Missing required values
  - `UnexpectedToken` - Parsing errors with position
  - `CommandMismatch` - Command name mismatches
  - `Internal` - Internal errors
- **Result type** - Proper error propagation throughout the API

#### Display System

- **Beautiful help output** with:
  - Formatted tables using box-drawing characters
  - Color-coded sections
  - Automatic usage pattern generation
  - Options and subcommands tables
  - Support for both root and subcommand help
- **Debug mode** via `.with_debug()` and `.add_debug_option()`
- **Error formatting** with detailed messages and suggestions
- **"Did you mean?"** suggestions for unknown commands using Levenshtein distance

#### API Improvements

- **Builder pattern** with method chaining
- **Separate methods** for different concerns:
  - `.add_option()` - Add options
  - `.set_callback()` - Set callbacks
  - `.command()` - Create commands
  - `.subcommand()` - Create subcommands
- **Expected positional args** - `.set_expected_positional_args(count)`

### Changed

#### Breaking Changes

##### Option Definition

**Before (v0.x):**

```rust
app.option("-n --name, <>", "Description", callback);
```

**After (v1.0):**

```rust
app.add_option("name", "Description", "-n", "--name",
    ValueTypes::RequiredSingle(Value::Str(String::new())));
app.set_callback(callback);
```

##### Callback Signature

**Before (v0.x):**

```rust
fn callback(app: &Fli) { }
```

**After (v1.0):**

```rust
fn callback(data: &FliCallbackData) { }
```

##### Value Retrieval

**Before (v0.x):**

```rust
let value = app.get_values("name".to_owned()).unwrap()[0];
```

**After (v1.0):**

```rust
let value = data.get_option_value("name")
    .and_then(|v| v.as_str())
    .unwrap_or("default");
```

##### Command Creation

**Before (v0.x):**

```rust
let cmd = app.command("serve", "Description");
```

**After (v1.0):**

```rust
let cmd = app.command("serve", "Description")?; // Returns Result
```

##### Error Types

**Before (v0.x):**

```rust
Result<(), String>
```

**After (v1.0):**

```rust
Result<(), FliError>
```

#### API Changes

- `.option()` renamed to `.add_option()` with new signature
- `.default()` renamed to `.set_callback()`
- `.run()` no longer panics, properly handles errors
- Removed string-based option template syntax (`"-n --name, <>"`)
- Commands now return `Result<&mut FliCommand>` instead of `&mut Fli`

### Improved

- **Help generation** - Dramatically improved with tables and sections
- **Error messages** - More descriptive with context and suggestions
- **Type safety** - Compile-time guarantees for option values
- **Documentation** - Comprehensive docs with examples
- **Performance** - More efficient parsing with state machine
- **Code organization** - Modular architecture with separate concerns

### Deprecated

- `init_from_toml()` method (use `init_fli_from_toml!()` macro instead)
- String-based option syntax (`"-n --name, <>"`)
- Direct `Fli` instance manipulation in callbacks

### Removed

- `.allow_duplicate_callback()` - No longer needed with new architecture
- `.allow_inital_no_param_values()` - Replaced by `.set_expected_positional_args()`
- `.get_values()` method on `Fli` - Use `FliCallbackData::get_option_value()` instead
- `.is_passed()` method on `Fli` - Check if `get_option_value()` returns `Some`
- `.has_a_value()` method on `Fli` - Use `get_option_value()` and check value type
- `.get_arg_at()` method on `Fli` - Use `FliCallbackData::get_argument_at()` instead
- `.print_help()` on `Fli` instance - Help is now automatic via `--help` flag
- `.get_params_callback()` - Internal method no longer exposed
- `.get_callable_name()` - Internal method no longer exposed

### Testing

- **Comprehensive test suite** added with 110+ tests covering:
  - Error handling (12 tests)
  - Value types (19 tests)
  - Option parser (15 tests)
  - Display utilities (10 tests)
  - Command system (14 tests)
  - App functionality (13 tests)
  - Library functions (17 tests)
  - Macro functionality (8 tests)

### Internal Improvements

- **Modular architecture** with separated concerns:
  - `app.rs` - Application structure
  - `command.rs` - Command handling
  - `display.rs` - Output formatting
  - `error.rs` - Error types
  - `option_parser/` - Parsing logic
    - `input_parser.rs` - Argument parsing
    - `option_parser.rs` - Option management
    - `value_types.rs` - Type system
    - `parse_state.rs` - Parser state machine
- **State machine-based parser** for reliable argument parsing
- **HashMap-based lookups** for O(1) option access
- **Improved documentation** with comprehensive examples

### Migration Guide

See detailed migration instructions below for upgrading from v0.x to v1.0.

---

## [0.1.0] - 2024-XX-XX

### Added

- New `init_fli_from_toml!()` macro to initialize app from Cargo.toml
  - Reads package name, version, and description at compile time
  - Eliminates need for manual metadata entry

### Deprecated

- `init_from_toml()` method - Use `init_fli_from_toml!()` macro instead
  - Will be removed in next major release

## [0.0.10] - 2024-XX-XX

### Added

- Auto version option to print version using `--version` or `-v`
- Version information automatically included in help output

## [0.0.9] - 2024-XX-XX

### Fixed

- Environment variable issue in TOML reading
  - Now uses `env!` macro for compile-time environment variables
  - Changed from `std::env::var` (runtime) to `env!` (compile-time)
  - Fixes issues with Cargo.toml details not being available

## [0.0.8] - 2024-XX-XX

### Fixed

- Index out of bounds error when getting values
- Improved bounds checking in value retrieval

## [0.0.7] - 2024-XX-XX

### Added

- Support for setting app version via `.set_version()`
  - Version automatically set from Cargo.toml when using `init_from_toml()`
- Enhanced help message formatting
  - Lists similar commands when command not found (Levenshtein distance)
  - Improved help screen layout and readability

### Improved

- Help message handling with better error context
- Command suggestion algorithm for typos

---

## Version Comparison

| Feature          | v0.x              | v1.0                 |
| ---------------- | ----------------- | -------------------- |
| Type Safety      | ❌ String-based   | ✅ Enum-based        |
| Error Handling   | ❌ String errors  | ✅ Typed errors      |
| Subcommands      | ⚠️ Limited        | ✅ Full support      |
| Help System      | ⚠️ Basic          | ✅ Beautiful tables  |
| Value Extraction | ⚠️ Manual parsing | ✅ Type-safe methods |
| Documentation    | ⚠️ Basic          | ✅ Comprehensive     |

[1.0.0]: https://github.com/codad5/fli/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/codad5/fli/compare/v0.0.10...v0.1.0
[0.0.10]: https://github.com/codad5/fli/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/codad5/fli/compare/v0.0.8...v0.0.9
[0.0.8]: https://github.com/codad5/fli/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/codad5/fli/releases/tag/v0.0.7
