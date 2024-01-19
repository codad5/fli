use fli::Fli;

// The commit messaege should in this format "fli-[<commit-reason(feature, bug, etc)>] [<commit-message>]"
// Example: fli-[feature] [add a new feature]
fn main() {
    let mut app = Fli::init("Apper", "An app that does many things");
     app.option("-l --list-apps", "To list all app", |_|{
        let mut app_list: Vec<[&str;2]> = Vec::new();
        app_list.push(["greeter", "To Greet"]);
        app_list.push(["repeater", "to repeatedly say something"]);
        for [name, des] in app_list {
            println!("{name}:{des}");
        }
        
    });


     app.command("greet", "An app that respects")
        .default(greet)
        .allow_inital_no_param_values(false)
        .option("-n --name, <>", "To print your name along side", greet)
        .option("-t --time, []", "For time based Greeting", greet);
    

    app.command("repeater", "To repeatedly say something")
    .allow_duplicate_callback(false)
    .allow_inital_no_param_values(true)
    .default(repeater)
    .option("-t --times, <>", "Number of times to repeat", repeater);
    
    // app.allow_duplicate_callback(true);
    app.run();
}

fn greet(x: &Fli){
    let name : String = match x.get_values("-n".to_string()){
        Ok(values) => values.get(0).unwrap().to_owned(),
        Err(_) => String::new()
    };
    let time : String = match x.get_values("-t".to_string()){
        Ok(values) => values.get(0).unwrap().to_owned(),
        Err(_) => String::from("Hello")
    };
    let time_saying : String = match time {
        _ => String::from("Hello")
    };
    println!("{time_saying} {name}")
}

fn repeater(x: &Fli)
{
    println!("running repeater");
    let message = x.get_arg_at(1).unwrap();
    let time : String = match x.get_values("-t".to_string()){
        Ok(values) => values.get(0).unwrap().to_owned(),
        Err(_) => "5".to_string()
    };
    let time = time.parse::<u8>().unwrap();
    for _ in 0..time{
        println!("{message}");
    }

}
