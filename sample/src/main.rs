/*
 * FileManager CLI - A comprehensive file system management tool
 *
 * This is a demonstration application showcasing the capabilities of the Fli CLI library.
 * It simulates common file system operations like ls, mkdir, rm, cp, mv, etc.
 *
 * Features demonstrated:
 * - Command hierarchies with subcommands
 * - Multiple value types (String, Integer, Boolean, Float)
 * - Required and optional options
 * - Single and multiple value arguments
 * - Positional arguments
 * - Callback data access
 * - Error handling
 * - Flag options (boolean switches)
 * - Preserved options (--help, --version)
 */

use colored::Colorize;
use fli::{
    init_fli_from_toml,
    option_parser::{Value, ValueTypes},
};
use std::fs;
use std::path::Path;

fn main() {
    // Initialize the CLI app from Cargo.toml with debug mode enabled
    let mut app = init_fli_from_toml!();
    app.add_debug_option();

    // ============================================================================
    // ROOT-LEVEL OPTIONS (available to all commands)
    // ============================================================================

    app.add_option(
        "verbose",
        "Enable verbose output for all operations",
        "-v",
        "--verbose",
        ValueTypes::None, // Flag option (no value required)
    );

    app.add_option(
        "quiet",
        "Suppress all non-error output",
        "-q",
        "--quiet",
        ValueTypes::None,
    );

    app.add_option(
        "color",
        "Enable/disable colored output (true/false)",
        "-c",
        "--color",
        ValueTypes::OptionalSingle(Some(Value::Bool(true))), // Optional with default
    );

    // ============================================================================
    // MARK OPTIONS AS INHERITABLE
    // ============================================================================
    // These options will be automatically available in all subcommands
    // This eliminates the need to redefine common options for each command
    app.mark_inheritable_many(&["-v", "-q", "-c"])
        .expect("Failed to mark options as inheritable");

    // ============================================================================
    // COMMAND: ls - List directory contents
    // ============================================================================

    app.command("ls", "List directory contents")
        .unwrap()
        .add_option(
            "all",
            "Show hidden files (starting with .)",
            "-a",
            "--all",
            ValueTypes::None,
        )
        .add_option(
            "long",
            "Use long listing format with details",
            "-l",
            "--long",
            ValueTypes::None,
        )
        .add_option(
            "human",
            "Print human-readable sizes (e.g., 1K, 234M)",
            "-h",
            "--human-readable",
            ValueTypes::None,
        )
        .add_option(
            "sort",
            "Sort by: name, size, time, or extension",
            "-s",
            "--sort",
            ValueTypes::OptionalSingle(Some(Value::Str("name".to_string()))),
        )
        .set_expected_positional_args(1) // Optional path argument
        .set_callback(|data| {
            // Get the path from positional arguments or use current directory
            let path = data.get_argument_at(0).map(|s| s.as_str()).unwrap_or(".");

            // Check if flags are set
            let show_all = data.get_option_value("all").is_some();
            let long_format = data.get_option_value("long").is_some();
            let human_readable = data.get_option_value("human").is_some();
            let verbose = data.get_option_value("verbose").is_some();

            // Get sort option
            let sort_by = data
                .get_option_value("sort")
                .and_then(|v| v.as_str())
                .unwrap_or("name");

            if verbose {
                println!("{} Listing directory: {}", "→".cyan(), path.yellow());
                println!("{} Sort by: {}", "→".cyan(), sort_by.yellow());
            }

            // Read directory
            match fs::read_dir(path) {
                Ok(entries) => {
                    let mut items: Vec<_> = entries.filter_map(Result::ok).collect();

                    // Sort entries
                    match sort_by {
                        "size" => items.sort_by_key(|e| e.metadata().map(|m| m.len()).unwrap_or(0)),
                        "time" => {
                            items.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok())
                        }
                        "extension" => items.sort_by_key(|e| {
                            e.path()
                                .extension()
                                .and_then(|s| s.to_str())
                                .unwrap_or("")
                                .to_string()
                        }),
                        _ => items.sort_by_key(|e| e.file_name()),
                    }

                    // Display entries
                    for entry in items {
                        let file_name = entry.file_name();
                        let name = file_name.to_string_lossy();

                        // Skip hidden files unless --all is specified
                        if !show_all && name.starts_with('.') {
                            continue;
                        }

                        if long_format {
                            // Long format with metadata
                            if let Ok(metadata) = entry.metadata() {
                                let size = if human_readable {
                                    format_size(metadata.len())
                                } else {
                                    metadata.len().to_string()
                                };

                                let file_type = if metadata.is_dir() {
                                    "DIR ".blue()
                                } else {
                                    "FILE".green()
                                };

                                println!("{} {:>10}  {}", file_type, size, name);
                            }
                        } else {
                            // Simple format
                            if entry.path().is_dir() {
                                println!("{}/", name.blue());
                            } else {
                                println!("{}", name);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("{} Failed to read directory: {}", "✗".red(), e),
            }
        });

    // ============================================================================
    // COMMAND: mkdir - Create directories
    // ============================================================================

    app.command("mkdir", "Create one or more directories")
        .unwrap()
        .add_option(
            "parents",
            "Create parent directories as needed (like mkdir -p)",
            "-p",
            "--parents",
            ValueTypes::None,
        )
        .add_option(
            "mode",
            "Set permission mode (octal, e.g., 755)",
            "-m",
            "--mode",
            ValueTypes::OptionalSingle(Some(Value::Int(755))),
        )
        .set_expected_positional_args(1) // At least one directory name required
        .set_callback(|data| {
            let dirs = data.get_arguments();
            let create_parents = data.get_option_value("parents").is_some();
            let verbose = data.get_option_value("verbose").is_some();

            if dirs.is_empty() {
                eprintln!("{} No directory specified", "✗".red());
                return;
            }

            for dir in dirs {
                let path = Path::new(&dir);

                let result = if create_parents {
                    fs::create_dir_all(path)
                } else {
                    fs::create_dir(path)
                };

                match result {
                    Ok(_) => {
                        if verbose {
                            println!("{} Created directory: {}", "✓".green(), dir.yellow());
                        }
                    }
                    Err(e) => eprintln!("{} Failed to create '{}': {}", "✗".red(), dir, e),
                }
            }
        });

    // ============================================================================
    // COMMAND: rm - Remove files and directories
    // ============================================================================

    app.command("rm", "Remove files or directories")
        .unwrap()
        .add_option(
            "recursive",
            "Remove directories and their contents recursively",
            "-r",
            "--recursive",
            ValueTypes::None,
        )
        .add_option(
            "force",
            "Ignore nonexistent files, never prompt",
            "-f",
            "--force",
            ValueTypes::None,
        )
        .add_option(
            "interactive",
            "Prompt before every removal",
            "-i",
            "--interactive",
            ValueTypes::None,
        )
        .set_expected_positional_args(1)
        .set_callback(|data| {
            let paths = data.get_arguments();
            let recursive = data.get_option_value("recursive").is_some();
            let force = data.get_option_value("force").is_some();
            let interactive = data.get_option_value("interactive").is_some();
            let verbose = data.get_option_value("verbose").is_some();

            if paths.is_empty() {
                eprintln!("{} No path specified", "✗".red());
                return;
            }

            for path_str in paths {
                let path = Path::new(&path_str);

                if !path.exists() {
                    if !force {
                        eprintln!(
                            "{} Cannot remove '{}': No such file or directory",
                            "✗".red(),
                            path_str
                        );
                    }
                    continue;
                }

                // Interactive confirmation
                if interactive && !force {
                    print!("Remove '{}'? (y/N): ", path_str);
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    if !input.trim().eq_ignore_ascii_case("y") {
                        continue;
                    }
                }

                let result = if path.is_dir() {
                    if recursive {
                        fs::remove_dir_all(path)
                    } else {
                        fs::remove_dir(path)
                    }
                } else {
                    fs::remove_file(path)
                };

                match result {
                    Ok(_) => {
                        if verbose {
                            println!("{} Removed: {}", "✓".green(), path_str.yellow());
                        }
                    }
                    Err(e) => {
                        if !force {
                            eprintln!("{} Failed to remove '{}': {}", "✗".red(), path_str, e);
                        }
                    }
                }
            }
        });

    // ============================================================================
    // COMMAND: cp - Copy files and directories
    // ============================================================================

    app.command("cp", "Copy files or directories")
        .unwrap()
        .add_option(
            "recursive",
            "Copy directories recursively",
            "-r",
            "--recursive",
            ValueTypes::None,
        )
        .add_option(
            "force",
            "Overwrite existing files without prompting",
            "-f",
            "--force",
            ValueTypes::None,
        )
        .add_option(
            "preserve",
            "Preserve file attributes (timestamps, permissions)",
            "-p",
            "--preserve",
            ValueTypes::None,
        )
        .set_expected_positional_args(2) // source(s) and destination
        .set_callback(|data| {
            let args = data.get_arguments();
            let recursive = data.get_option_value("recursive").is_some();
            let force = data.get_option_value("force").is_some();
            let verbose = data.get_option_value("verbose").is_some();

            if args.len() < 2 {
                eprintln!("{} Usage: cp [OPTIONS] SOURCE... DEST", "✗".red());
                return;
            }

            let dest = &args[args.len() - 1];
            let sources = &args[..args.len() - 1];

            for source in sources {
                let src_path = Path::new(source);
                let dest_path = Path::new(dest);

                if !src_path.exists() {
                    eprintln!("{} Source '{}' does not exist", "✗".red(), source);
                    continue;
                }

                // Determine final destination path
                let final_dest = if dest_path.is_dir() {
                    dest_path.join(src_path.file_name().unwrap())
                } else {
                    dest_path.to_path_buf()
                };

                // Check if destination exists
                if final_dest.exists() && !force {
                    eprintln!(
                        "{} Destination '{}' already exists (use -f to overwrite)",
                        "✗".red(),
                        final_dest.display()
                    );
                    continue;
                }

                let result = if src_path.is_dir() {
                    if !recursive {
                        eprintln!(
                            "{} '{}' is a directory (use -r for recursive copy)",
                            "✗".red(),
                            source
                        );
                        continue;
                    }
                    copy_dir_recursive(src_path, &final_dest)
                } else {
                    fs::copy(src_path, &final_dest).map(|_| ())
                };

                match result {
                    Ok(_) => {
                        if verbose {
                            println!(
                                "{} Copied '{}' to '{}'",
                                "✓".green(),
                                source.yellow(),
                                final_dest.display().to_string().yellow()
                            );
                        }
                    }
                    Err(e) => eprintln!("{} Failed to copy '{}': {}", "✗".red(), source, e),
                }
            }
        });

    // ============================================================================
    // COMMAND: mv - Move/rename files and directories
    // ============================================================================

    app.command("mv", "Move or rename files and directories")
        .unwrap()
        .add_option(
            "force",
            "Overwrite existing files without prompting",
            "-f",
            "--force",
            ValueTypes::None,
        )
        .add_option(
            "interactive",
            "Prompt before overwriting",
            "-i",
            "--interactive",
            ValueTypes::None,
        )
        .add_option(
            "no-clobber",
            "Don't overwrite existing files",
            "-n",
            "--no-clobber",
            ValueTypes::None,
        )
        .set_expected_positional_args(2)
        .set_callback(|data| {
            let args = data.get_arguments();
            let force = data.get_option_value("force").is_some();
            let interactive = data.get_option_value("interactive").is_some();
            let no_clobber = data.get_option_value("no-clobber").is_some();
            let verbose = data.get_option_value("verbose").is_some();

            if args.len() < 2 {
                eprintln!("{} Usage: mv [OPTIONS] SOURCE DEST", "✗".red());
                return;
            }

            let source = &args[0];
            let dest = &args[1];

            let src_path = Path::new(source);
            let dest_path = Path::new(dest);

            if !src_path.exists() {
                eprintln!("{} Source '{}' does not exist", "✗".red(), source);
                return;
            }

            if dest_path.exists() {
                if no_clobber {
                    eprintln!("{} Destination '{}' already exists", "✗".red(), dest);
                    return;
                }

                if interactive && !force {
                    print!("Overwrite '{}'? (y/N): ", dest);
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    if !input.trim().eq_ignore_ascii_case("y") {
                        return;
                    }
                }
            }

            match fs::rename(src_path, dest_path) {
                Ok(_) => {
                    if verbose {
                        println!(
                            "{} Moved '{}' to '{}'",
                            "✓".green(),
                            source.yellow(),
                            dest.yellow()
                        );
                    }
                }
                Err(e) => eprintln!("{} Failed to move '{}': {}", "✗".red(), source, e),
            }
        });

    // ============================================================================
    // COMMAND: cat - Display file contents
    // ============================================================================

    app.command("cat", "Display file contents")
        .unwrap()
        .add_option(
            "number",
            "Number all output lines",
            "-n",
            "--number",
            ValueTypes::None,
        )
        .add_option(
            "show-ends",
            "Display $ at end of each line",
            "-E",
            "--show-ends",
            ValueTypes::None,
        )
        .set_expected_positional_args(1)
        .set_callback(|data| {
            let files = data.get_arguments();
            let number_lines = data.get_option_value("number").is_some();
            let show_ends = data.get_option_value("show-ends").is_some();

            if files.is_empty() {
                eprintln!("{} No file specified", "✗".red());
                return;
            }

            for file in files {
                match fs::read_to_string(file) {
                    Ok(contents) => {
                        for (i, line) in contents.lines().enumerate() {
                            if number_lines {
                                print!("{:6} ", (i + 1).to_string().cyan());
                            }
                            print!("{}", line);
                            if show_ends {
                                print!("{}", "$".blue());
                            }
                            println!();
                        }
                    }
                    Err(e) => eprintln!("{} Failed to read '{}': {}", "✗".red(), file, e),
                }
            }
        });

    // ============================================================================
    // COMMAND: find - Search for files
    // ============================================================================

    app.command("find", "Search for files and directories")
        .unwrap()
        .add_option(
            "name",
            "Search by filename pattern (supports wildcards)",
            "-n",
            "--name",
            ValueTypes::OptionalSingle(Some(Value::Str("*".to_string()))),
        )
        .add_option(
            "type",
            "Filter by type: f (file), d (directory)",
            "-t",
            "--type",
            ValueTypes::OptionalSingle(None),
        )
        .add_option(
            "size",
            "Filter by size in bytes (prefix with +/- for greater/less than)",
            "-s",
            "--size",
            ValueTypes::OptionalSingle(None),
        )
        .add_option(
            "max-depth",
            "Maximum directory depth to search",
            "-d",
            "--max-depth",
            ValueTypes::OptionalSingle(Some(Value::Int(10))),
        )
        .set_expected_positional_args(0)
        .set_callback(|data| {
            let path = data.get_argument_at(0).map(|s| s.as_str()).unwrap_or(".");
            let name_pattern = data
                .get_option_value("name")
                .and_then(|v| v.as_str())
                .unwrap_or("*");
            let file_type = data.get_option_value("type").and_then(|v| v.as_str());
            let verbose = data.get_option_value("verbose").is_some();

            if verbose {
                println!("{} Searching in: {}", "→".cyan(), path.yellow());
                println!("{} Pattern: {}", "→".cyan(), name_pattern.yellow());
            }

            search_files(Path::new(path), name_pattern, file_type, 0, 5);
        });

    // ============================================================================
    // COMMAND: stat - Display file statistics
    // ============================================================================

    app.command("stat", "Display detailed file or directory information")
        .unwrap()
        .add_option(
            "format",
            "Output format: text or json",
            "-f",
            "--format",
            ValueTypes::OptionalSingle(Some(Value::Str("text".to_string()))),
        )
        .set_expected_positional_args(1)
        .set_callback(|data| {
            let files = data.get_arguments();
            let format = data
                .get_option_value("format")
                .and_then(|v| v.as_str())
                .unwrap_or("text");

            if files.is_empty() {
                eprintln!("{} No file specified", "✗".red());
                return;
            }

            for file in files {
                let path = Path::new(&file);

                match fs::metadata(path) {
                    Ok(metadata) => {
                        if format == "json" {
                            println!("{{");
                            println!("  \"path\": \"{}\",", file);
                            println!(
                                "  \"type\": \"{}\",",
                                if metadata.is_dir() {
                                    "directory"
                                } else {
                                    "file"
                                }
                            );
                            println!("  \"size\": {},", metadata.len());
                            println!("  \"readonly\": {}", metadata.permissions().readonly());
                            println!("}}");
                        } else {
                            println!("{}", "═".repeat(60).cyan());
                            println!("{}: {}", "File".bold(), file.yellow());
                            println!("{}", "─".repeat(60).cyan());
                            println!(
                                "{}: {}",
                                "Type".bold(),
                                if metadata.is_dir() {
                                    "Directory".blue()
                                } else {
                                    "File".green()
                                }
                            );
                            println!(
                                "{}: {} bytes ({})",
                                "Size".bold(),
                                metadata.len(),
                                format_size(metadata.len()).yellow()
                            );
                            println!(
                                "{}: {}",
                                "Read-only".bold(),
                                if metadata.permissions().readonly() {
                                    "Yes".red()
                                } else {
                                    "No".green()
                                }
                            );

                            if let Ok(modified) = metadata.modified() {
                                println!("{}: {:?}", "Modified".bold(), modified);
                            }
                            if let Ok(accessed) = metadata.accessed() {
                                println!("{}: {:?}", "Accessed".bold(), accessed);
                            }
                            if let Ok(created) = metadata.created() {
                                println!("{}: {:?}", "Created".bold(), created);
                            }
                            println!("{}", "═".repeat(60).cyan());
                        }
                    }
                    Err(e) => {
                        eprintln!("{} Failed to get metadata for '{}': {}", "✗".red(), file, e)
                    }
                }
            }
        });

    // ============================================================================
    // COMMAND: tree - Display directory tree
    // ============================================================================

    app.command("tree", "Display directory tree structure")
        .unwrap()
        .add_option("all", "Show hidden files", "-a", "--all", ValueTypes::None)
        .add_option(
            "dirs-only",
            "List directories only",
            "-d",
            "--dirs-only",
            ValueTypes::None,
        )
        .add_option(
            "level",
            "Maximum display depth",
            "-L",
            "--level",
            ValueTypes::OptionalSingle(Some(Value::Int(3))),
        )
        .set_expected_positional_args(0)
        .set_callback(|data| {
            let path = data.get_argument_at(0).map(|s| s.as_str()).unwrap_or(".");
            let show_all = data.get_option_value("all").is_some();
            let dirs_only = data.get_option_value("dirs-only").is_some();
            let max_level = data
                .get_option_value("level")
                .and_then(|v| match v {
                    ValueTypes::OptionalSingle(Some(Value::Int(n))) => Some(*n as usize),
                    _ => None,
                })
                .unwrap_or(3);

            println!("{}", path.cyan().bold());
            display_tree(Path::new(path), "", show_all, dirs_only, 0, max_level);
        });

    // Run the application
    app.run();
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Format bytes into human-readable size (KB, MB, GB, etc.)
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

/// Recursively copy a directory
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

/// Search for files matching a pattern
fn search_files(
    path: &Path,
    pattern: &str,
    file_type: Option<&str>,
    depth: usize,
    max_depth: usize,
) {
    if depth > max_depth {
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            // Skip hidden files unless pattern starts with .
            if name.starts_with('.') && !pattern.starts_with('.') {
                continue;
            }

            let is_dir = entry_path.is_dir();

            // Apply type filter
            if let Some(ft) = file_type {
                if (ft == "f" && is_dir) || (ft == "d" && !is_dir) {
                    if is_dir {
                        search_files(&entry_path, pattern, file_type, depth + 1, max_depth);
                    }
                    continue;
                }
            }

            // Simple wildcard matching
            let matches = if pattern == "*" {
                true
            } else if pattern.contains('*') {
                let pattern_parts: Vec<&str> = pattern.split('*').collect();
                let mut name_str = name.as_ref();
                let mut matched = true;

                for (i, part) in pattern_parts.iter().enumerate() {
                    if part.is_empty() {
                        continue;
                    }
                    if i == 0 && !name_str.starts_with(part) {
                        matched = false;
                        break;
                    }
                    if let Some(pos) = name_str.find(part) {
                        name_str = &name_str[pos + part.len()..];
                    } else {
                        matched = false;
                        break;
                    }
                }
                matched
            } else {
                name.contains(pattern)
            };

            if matches {
                if is_dir {
                    println!("{}/", entry_path.display().to_string().blue());
                } else {
                    println!("{}", entry_path.display());
                }
            }

            if is_dir {
                search_files(&entry_path, pattern, file_type, depth + 1, max_depth);
            }
        }
    }
}

/// Display directory tree structure
fn display_tree(
    path: &Path,
    prefix: &str,
    show_all: bool,
    dirs_only: bool,
    level: usize,
    max_level: usize,
) {
    if level >= max_level {
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        let mut items: Vec<_> = entries.filter_map(Result::ok).collect();
        items.sort_by_key(|e| e.file_name());

        let count = items.len();

        for (i, entry) in items.iter().enumerate() {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            // Skip hidden files unless --all is specified
            if !show_all && name.starts_with('.') {
                continue;
            }

            let is_last = i == count - 1;
            let connector = if is_last { "└── " } else { "├── " };
            let extension = if is_last { "    " } else { "│   " };

            let path = entry.path();
            let is_dir = path.is_dir();

            // Skip files if dirs-only is set
            if dirs_only && !is_dir {
                continue;
            }

            if is_dir {
                println!("{}{}{}/", prefix, connector, name.to_string().blue());
                display_tree(
                    &path,
                    &format!("{}{}", prefix, extension),
                    show_all,
                    dirs_only,
                    level + 1,
                    max_level,
                );
            } else {
                println!("{}{}{}", prefix, connector, name);
            }
        }
    }
}
