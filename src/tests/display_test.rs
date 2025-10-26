use crate::display::{
    debug_print, debug_struct, disable_debug, enable_debug, is_debug_enabled, TableStyle,
};

#[test]
fn test_debug_enabled_default() {
    // Debug should be disabled by default
    disable_debug();
    assert!(!is_debug_enabled());
}

#[test]
fn test_enable_debug() {
    enable_debug();
    assert!(is_debug_enabled());
    disable_debug(); // Clean up
}

#[test]
fn test_disable_debug() {
    enable_debug();
    disable_debug();
    assert!(!is_debug_enabled());
}

#[test]
fn test_debug_toggle() {
    disable_debug();
    assert!(!is_debug_enabled());

    enable_debug();
    assert!(is_debug_enabled());

    disable_debug();
    assert!(!is_debug_enabled());
}

#[test]
fn test_debug_print_when_disabled() {
    disable_debug();
    // This should not panic or output anything
    debug_print("Test", "Message");
}

#[test]
fn test_debug_print_when_enabled() {
    enable_debug();
    // This should output debug info (visually verify if needed)
    debug_print("Test", "Message");
    disable_debug(); // Clean up
}

#[test]
fn test_debug_struct_when_disabled() {
    disable_debug();
    let data = vec![1, 2, 3];
    // This should not panic or output anything
    debug_struct("TestData", &data);
}

#[test]
fn test_debug_struct_when_enabled() {
    enable_debug();
    let data = vec![1, 2, 3];
    // This should output debug info (visually verify if needed)
    debug_struct("TestData", &data);
    disable_debug(); // Clean up
}

#[test]
fn test_table_style_default() {
    let style = TableStyle::default();
    assert_eq!(style.padding, 2);
    assert!(style.show_borders);
}

#[test]
fn test_table_style_clone() {
    let style1 = TableStyle::default();
    let style2 = style1.clone();

    assert_eq!(style1.padding, style2.padding);
    assert_eq!(style1.show_borders, style2.show_borders);
}

#[test]
fn test_table_style_custom() {
    use colored::Color;

    let style = TableStyle {
        header_color: Color::Red,
        border_color: Color::Green,
        padding: 4,
        show_borders: false,
    };

    assert_eq!(style.padding, 4);
    assert!(!style.show_borders);
}
