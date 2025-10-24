use fli::{app::Fli, option_parser::{Value, ValueTypes}};


fn main() {
    let mut app = Fli::new("myapp", "1.0.0", "A sample CLI app").with_debug();
    
    // Define root-level options
    app.add_option("verbose", "Enable verbose output", "-v", "--verbose", 
        ValueTypes::None);
    
    app.add_option("config", "Config file path", "-c", "--config", 
        ValueTypes::OptionalSingle(None));
    
    // Define a command with options
    let greet_cmd = app.command("greet", "Greet someone").unwrap()
        .add_option("name", "Name to greet", "-n", "--name", 
            ValueTypes::RequiredSingle(Value::Str(String::new())))
        .add_option("times", "Times to greet", "-t", "--times", 
            ValueTypes::OptionalSingle(Some(Value::Int(1))))
        .set_expected_positional_args(2)
        .set_callback(|data| {
            let name = data.get_option_value("name")
                .and_then(|v| v.as_str())
                .unwrap_or("World");
            
            let times = data.get_option_value("times")
                .and_then(|v| match v {
                    ValueTypes::OptionalSingle(Some(Value::Int(n))) => Some(*n),
                    _ => None,
                })
                .unwrap_or(1);
            
            for _ in 0..times {
                println!("Hello, {}!", name);
            }
        });
    
    // Define a file command with subcommands
    let file_cmd = app.command("file", "File operations").unwrap();
    
    file_cmd.subcommand("copy", "Copy files")
        .add_option("source", "Source files", "-s", "--source", 
            ValueTypes::RequiredMultiple(vec![], None))
        .add_option("dest", "Destination", "-d", "--dest", 
            ValueTypes::RequiredSingle(Value::Str(String::new())))
        .set_callback(|data| {
            println!("data: source: {:?}", data.get_option_value("source"));
            let sources = data.get_option_value("source")
                .and_then(|v| v.as_strings())
                .unwrap_or_default();
            
            let dest = data.get_option_value("dest")
                .and_then(|v| v.as_str())
                .unwrap_or(".");
            
            println!("Copying {:?} to {}", sources, dest);
        });
    
    // Run the app
    app.run();
}