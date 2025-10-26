# FileManager CLI - Quick Usage Examples

This document provides quick copy-paste examples for testing all features of the FileManager CLI.

## Setup

```bash
cd sample
cargo build --release
alias fm='./target/release/sample'
```

## Basic Commands

### 1. List Directory (ls)

```bash
# Simple list
fm ls

# List with details
fm ls -l

# Show all files including hidden
fm ls -a

# Human-readable sizes
fm ls -lh

# Sort by size
fm ls -l --sort=size

# List specific directory
fm ls src/

# Combined flags
fm ls -alh --sort=time
```

### 2. Create Directories (mkdir)

```bash
# Create single directory
fm mkdir test1

# Create nested directories
fm mkdir -p path/to/deep/directory

# Create multiple directories
fm mkdir dir1 dir2 dir3

# With verbose output
fm -v mkdir -p deep/nested/path

# With mode (permission)
fm mkdir --mode 755 new_dir
```

### 3. Remove Files/Directories (rm)

```bash
# Remove a file
fm rm test.txt

# Remove directory (requires -r)
fm rm -r test_dir

# Force remove without prompts
fm rm -rf old_directory

# Interactive mode
fm rm -i file1.txt file2.txt

# Verbose removal
fm -v rm -r temp/
```

### 4. Copy Files (cp)

```bash
# Copy a file
fm cp file1.txt file2.txt

# Copy to directory
fm cp file.txt target_dir/

# Copy directory recursively
fm cp -r source_dir backup_dir

# Force overwrite
fm cp -f source.txt existing.txt

# Copy multiple files
fm cp file1.txt file2.txt file3.txt dest_dir/

# Verbose copy
fm -v cp -r project/ backup/
```

### 5. Move/Rename Files (mv)

```bash
# Rename a file
fm mv old_name.txt new_name.txt

# Move to directory
fm mv file.txt target_dir/

# Force move (overwrite)
fm mv -f source.txt destination.txt

# Interactive move
fm mv -i file.txt existing.txt

# No clobber (don't overwrite)
fm mv -n source.txt destination.txt
```

### 6. Display File Contents (cat)

```bash
# Display file
fm cat README.md

# Display multiple files
fm cat file1.txt file2.txt

# With line numbers
fm cat -n file.txt

# Show line endings
fm cat -E file.txt

# Combined options
fm cat -nE source_code.rs
```

### 7. Find Files (find)

```bash
# Find all files in current directory
fm find

# Find in specific directory
fm find src/

# Search by name pattern
fm find --name "*.txt"

# Find only directories
fm find --type d

# Find only files
fm find --type f

# Limit search depth
fm find --max-depth 2

# Complex search
fm find . --name "*.rs" --type f
```

### 8. File Statistics (stat)

```bash
# Show file info
fm stat README.md

# JSON output
fm stat --format json Cargo.toml

# Multiple files
fm stat file1.txt file2.txt src/

# Directory stats
fm stat target/
```

### 9. Directory Tree (tree)

```bash
# Display current directory tree
fm tree

# Show hidden files
fm tree -a

# Directories only
fm tree -d

# Limit depth
fm tree --level 2

# Specific directory
fm tree src/

# Combined options
fm tree -ad --level 3
```

## Advanced Usage

### Combining Global Options

```bash
# Verbose mode with any command
fm -v ls -l
fm -v mkdir test_dir
fm -v cp -r src/ backup/

# Quiet mode
fm -q mkdir temp/

# Disable colors
fm --color=false ls -l

# Debug mode
fm --debug ls
```

### Practical Workflows

#### Create Project Structure

```bash
fm mkdir -p project/src project/tests project/docs
fm tree project/
```

#### Backup Files

```bash
fm mkdir backups
fm cp -r important_files/ backups/
fm ls -lh backups/
```

#### Clean Up

```bash
fm find . --name "*.tmp" --type f
fm rm -f *.tmp
```

#### File Management

```bash
# Create test files
fm mkdir test_area
cd test_area
touch file1.txt file2.txt file3.txt

# List them
fm ls -l

# Copy with pattern
fm cp *.txt backup/

# Move files
fm mv file1.txt renamed.txt

# Check result
fm tree
```

### Pipeline Examples

```bash
# Find and list large directories
fm find --type d | fm ls -lh

# Create backup structure
fm tree src/ --dirs-only | fm mkdir -p backup/

# Organize files by extension
fm find --name "*.txt" --type f | fm cp backup/txt/
fm find --name "*.rs" --type f | fm cp backup/rust/
```

## Testing All Features

Run this script to test all major features:

```bash
#!/bin/bash

echo "=== Testing FileManager CLI ==="

echo -e "\n1. Creating test structure..."
fm mkdir -p test/dir1 test/dir2 test/dir3

echo -e "\n2. Creating test files..."
echo "Test content" > test/file1.txt
echo "More content" > test/file2.txt

echo -e "\n3. Listing with details..."
fm ls -lh test/

echo -e "\n4. Displaying tree..."
fm tree test/ --level 2

echo -e "\n5. Copying files..."
fm cp -r test/ test_backup/

echo -e "\n6. Finding files..."
fm find test/ --name "*.txt"

echo -e "\n7. Displaying file stats..."
fm stat test/file1.txt

echo -e "\n8. Showing file content..."
fm cat -n test/file1.txt

echo -e "\n9. Moving files..."
fm mv test/file2.txt test/renamed.txt

echo -e "\n10. Final tree view..."
fm tree test/

echo -e "\n11. Cleanup..."
fm rm -rf test/ test_backup/

echo -e "\n=== All tests completed ==="
```

## Error Handling Examples

```bash
# Try to remove non-existent file
fm rm nonexistent.txt
# Output: ✗ Cannot remove 'nonexistent.txt': No such file or directory

# Try to copy directory without -r
fm cp src/ backup/
# Output: ✗ 'src/' is a directory (use -r for recursive copy)

# Try to list non-existent directory
fm ls /nonexistent/
# Output: ✗ Failed to read directory: ...
```

## Performance Testing

```bash
# Large directory listing
time fm ls -l /usr/lib/

# Deep directory creation
time fm mkdir -p very/deep/nested/directory/structure/test

# Recursive copy
time fm cp -r large_directory/ backup/

# Extensive search
time fm find / --name "*.conf" --max-depth 5
```

## Help System

```bash
# General help
fm --help

# Command-specific help
fm ls --help
fm mkdir --help
fm cp --help
fm rm --help
fm mv --help
fm cat --help
fm find --help
fm stat --help
fm tree --help

# Version information
fm --version
```

## Tips and Tricks

1. **Always use -v (verbose) for important operations:**

   ```bash
   fm -v rm -rf important_data/  # Shows what's being deleted
   ```

2. **Use -i (interactive) for destructive operations:**

   ```bash
   fm rm -i *.txt  # Prompts before each deletion
   ```

3. **Combine find with other commands:**

   ```bash
   fm find --name "*.log" --type f | xargs fm rm -f
   ```

4. **Use stat for detailed file information:**

   ```bash
   fm stat --format json myfile.txt | jq .
   ```

5. **Tree is great for documentation:**

   ```bash
   fm tree --level 3 > project_structure.txt
   ```

6. **Always test with --dry-run equivalent (verbose mode):**
   ```bash
   fm -v cp -r important/ backup/  # Check what will be copied
   ```

## Troubleshooting

### Command not recognized

```bash
# Make sure you're in the correct directory
cd sample/
./target/release/sample --help
```

### Permission denied

```bash
# Check file permissions
fm stat file.txt

# Try with different mode
fm mkdir --mode 777 test_dir
```

### File not found

```bash
# Use find to locate
fm find / --name "filename" --max-depth 5

# Check current directory
fm ls -la
```

---

**Happy file managing! 📁✨**
