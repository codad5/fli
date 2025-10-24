use colored::*;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

// ==================== DEBUG CONTROL ====================

/// Global debug flag - controls whether debug output is shown
static DEBUG_ENABLED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

/// Enable debug output globally
pub fn enable_debug() {
    DEBUG_ENABLED.store(true, Ordering::Relaxed);
}

/// Disable debug output globally
pub fn disable_debug() {
    DEBUG_ENABLED.store(false, Ordering::Relaxed);
}

/// Check if debug output is enabled
pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

/// Print debug message if debug is enabled
///
/// # Examples
/// ```
/// debug_print("Parser state", &format!("{:?}", state));
/// ```
pub fn debug_print(label: &str, message: &str) {
    if is_debug_enabled() {
        eprintln!("{} {}: {}", 
            "[DEBUG]".bright_magenta().bold(),
            label.cyan(),
            message.white()
        );
    }
}

/// Print debug with structured data
pub fn debug_struct<T: std::fmt::Debug>(label: &str, data: &T) {
    if is_debug_enabled() {
        eprintln!("{} {}:\n{:#?}", 
            "[DEBUG]".bright_magenta().bold(),
            label.cyan(),
            data
        );
    }
}

// ==================== TABLE PRINTING ====================

/// Configuration for table appearance
#[derive(Clone)]
pub struct TableStyle {
    pub header_color: Color,
    pub border_color: Color,
    pub padding: usize,
    pub show_borders: bool,
}

impl Default for TableStyle {
    fn default() -> Self {
        Self {
            header_color: Color::Blue,
            border_color: Color::White,
            padding: 2,
            show_borders: true,
        }
    }
}

/// Print a formatted table to stdout
///
/// # Arguments
/// * `headers` - Column headers
/// * `rows` - Data rows (must have same length as headers)
/// * `style` - Optional style configuration
///
/// # Panics
/// Panics if any row length doesn't match header length
///
/// # Examples
/// ```
/// let headers = vec!["Name", "Age", "City"];
/// let rows = vec![
///     vec!["Alice", "30", "NYC"],
///     vec!["Bob", "25", "LA"],
/// ];
/// print_table(&headers, &rows, None);
/// ```
pub fn print_table(
    headers: &[&str],
    rows: &[Vec<&str>],
    style: Option<TableStyle>,
) {
    let style = style.unwrap_or_default();
    
    // Validate all rows have correct column count
    for (idx, row) in rows.iter().enumerate() {
        if row.len() != headers.len() {
            panic!(
                "Row {} has {} columns, but headers have {}",
                idx, row.len(), headers.len()
            );
        }
    }
    
    // Calculate column widths
    let mut col_widths: Vec<usize> = headers.iter()
        .map(|h| h.len())
        .collect();
    
    for row in rows {
        for (idx, cell) in row.iter().enumerate() {
            col_widths[idx] = col_widths[idx].max(cell.len());
        }
    }
    
    // Print header
    print_table_row(headers, &col_widths, style.padding, Some(style.header_color));
    
    // Print separator
    if style.show_borders {
        print_separator(&col_widths, style.padding, style.border_color);
    }
    
    // Print rows
    for row in rows {
        print_table_row(row, &col_widths, style.padding, None);
    }
}

fn print_table_row(cells: &[&str], widths: &[usize], padding: usize, color: Option<Color>) {
    let pad = " ".repeat(padding);
    print!("{}", pad);
    
    for (idx, cell) in cells.iter().enumerate() {
        let formatted = format!("{:<width$}", cell, width = widths[idx]);
        if let Some(c) = color {
            print!("{}", formatted.color(c));
        } else {
            print!("{}", formatted);
        }
        
        if idx < cells.len() - 1 {
            print!(" | ");
        }
    }
    println!();
}

fn print_separator(widths: &[usize], padding: usize, color: Color) {
    let pad = " ".repeat(padding);
    print!("{}", pad);
    
    for (idx, width) in widths.iter().enumerate() {
        print!("{}", "─".repeat(*width).color(color));
        if idx < widths.len() - 1 {
            print!("─┼─");
        }
    }
    println!();
}

// ==================== HELP SCREEN FORMATTING ====================

/// Print a section header
pub fn print_section(title: &str) {
    println!("\n{}", title.bold().blue());
}

/// Print usage line
pub fn print_usage(app_name: &str, pattern: &str) {
    println!("{}: {} {}", 
        "Usage".bold().yellow(),
        app_name.green(),
        pattern.white()
    );
}

/// Print app info header
pub fn print_app_header(name: &str, version: &str, description: &str) {
    println!("{}: {}", "Name".bold().green(), name);
    println!("{}: {}", "Version".bold().green(), version);
    println!("{}: {}", "Description".bold().blue(), description);
}

/// Print a list of items with descriptions
///
/// # Examples
/// ```
/// let items = vec![
///     ("serve", "Start the server"),
///     ("build", "Build the project"),
/// ];
/// print_item_list(&items, Some("Commands:"));
/// ```
pub fn print_item_list(items: &[(&str, &str)], title: Option<&str>) {
    if let Some(t) = title {
        print_section(t);
    }
    
    let max_width = items.iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0);
    
    for (name, desc) in items {
        println!("  {:<width$}  {}", 
            name.cyan(),
            desc.white(),
            width = max_width
        );
    }
}

// ==================== ERROR FORMATTING ====================

/// Print an error message with formatting
pub fn print_error(message: &str) {
    eprintln!("{} {}", "Error:".bold().red(), message.red());
}

/// Print error with detailed context
pub fn print_error_detailed(title: &str, message: &str, hint: Option<&str>) {
    eprintln!();
    eprintln!("{}", "═".repeat(60).red());
    eprintln!("{} {}", "ERROR:".bold().red(), title.bright_red());
    eprintln!("{}", "═".repeat(60).red());
    eprintln!();
    eprintln!("  {}", message.red());
    
    if let Some(h) = hint {
        eprintln!();
        eprintln!("{} {}", "Hint:".bold().yellow(), h.white());
    }
    
    eprintln!();
    eprintln!("{}", "═".repeat(60).red());
    eprintln!();
}

// ==================== SUCCESS/INFO MESSAGES ====================

/// Print success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".bold().green(), message.green());
}

/// Print info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".bold().blue(), message.white());
}

/// Print warning message
pub fn print_warning(message: &str) {
    eprintln!("{} {}", "⚠".bold().yellow(), message.yellow());
}

// ==================== SUGGESTIONS ====================

/// Print similar command suggestions
pub fn print_suggestions(unknown: &str, suggestions: &[String]) {
    if suggestions.is_empty() {
        return;
    }
    
    eprintln!();
    eprintln!("{} '{}'", "Unknown command:".red(), unknown.bright_red());
    eprintln!();
    eprintln!("{}", "Did you mean:".bold().yellow());
    
    for suggestion in suggestions {
        eprintln!("    {} {}", "•".cyan(), suggestion.green());
    }
    eprintln!();
}

// ==================== PROGRESS INDICATORS ====================

/// Print a progress indicator
pub fn print_progress(step: usize, total: usize, message: &str) {
    let percentage = (step as f64 / total as f64 * 100.0) as usize;
    let filled = percentage / 5;
    let empty = 20 - filled;
    
    print!("\r{} [{}{}>] {}% - {}", 
        "Progress:".bold().blue(),
        "█".repeat(filled).green(),
        " ".repeat(empty),
        percentage,
        message
    );
    
    if step == total {
        println!(); // New line when complete
    }
}

// ==================== BOX DRAWING ====================

/// Print text in a box
pub fn print_box(text: &str, color: Color) {
    let width = text.len() + 4;
    let top = format!("┌{}┐", "─".repeat(width - 2));
    let middle = format!("│ {} │", text);
    let bottom = format!("└{}┘", "─".repeat(width - 2));
    
    println!("{}", top.color(color));
    println!("{}", middle.color(color).bold());
    println!("{}", bottom.color(color));
}

// ==================== UTILITIES ====================

/// Print a horizontal divider
pub fn print_divider(width: usize, style: char, color: Option<Color>) {
    let line = style.to_string().repeat(width);
    if let Some(c) = color {
        println!("{}", line.color(c));
    } else {
        println!("{}", line);
    }
}

/// Print key-value pairs
pub fn print_key_value(pairs: &[(&str, &str)]) {
    let max_key_width = pairs.iter()
        .map(|(k, _)| k.len())
        .max()
        .unwrap_or(0);
    
    for (key, value) in pairs {
        println!("  {:<width$}: {}", 
            key.bold().cyan(),
            value.white(),
            width = max_key_width
        );
    }
}

// ==================== LEVENSHTEIN & SUGGESTIONS ====================

/// Calculate Levenshtein distance between two strings
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    
    if len1 == 0 { return len2; }
    if len2 == 0 { return len1; }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }
    
    matrix[len1][len2]
}

/// Find similar strings based on Levenshtein distance
pub fn find_similar<'a>(
    target: &str,
    candidates: &'a [String],
    max_distance: usize,
) -> Vec<&'a String> {
    candidates
        .iter()
        .filter(|candidate| levenshtein_distance(target, candidate) <= max_distance)
        .collect()
}

/// Print "did you mean" suggestions for unknown commands
pub fn print_did_you_mean(unknown: &str, available: &[String]) {
    let suggestions = find_similar(unknown, available, 2);
    let suggestion_vec: Vec<String> = suggestions.into_iter().cloned().collect();
    print_suggestions(unknown, &suggestion_vec);
}