use fli::{init_fli_from_toml, Fli};



// list of apps  
//  - Calculator 
//      - operand 
//      - num1 and num2
//  - Greating application
//      = name  required and string
//      - time   not required || morning , evening , afternoon  
//      - repeat not required || number
//  - String Counter 
//      - string required
//      - method not required || word , character 


fn main(){
    let mut app : Fli = init_fli_from_toml!();
    app.allow_inital_no_param_values(false);

    let mut calc_app = app.command("calc", "perform basic 2 numeric calculation");

    calc_app.option("-o --operand, <>", "Operand to perform", calc);


    let mut greeting_app = app.command("greet", "Greet your self");
    let mut strcounter = app.command("strcounter", "Count a str lenght");



    app.run();
}



fn calc(x : &Fli) {
    let operand : String = match x.get_values("operand".to_owned()){
        Ok(value) => value[0].to_owned(),
        Err(_) => "".to_owned(),
    };

    let valid_operand: Vec<&str> = vec!["a", "m", "s", "d", "r"];
    if !valid_operand.contains(&operand.as_str()){
        x.print_help("Invalid operand , use a => add, m => multiply, s => subtract, d => divide, r => remainder");
        return;
    }

    let val1 = match x.get_arg_at(3){
        Some(value) => check_for_valid_no(value.to_owned()),
        None => {
            x.print_help("Please provide 2 numeric values \n  usage: calc -o <operand> <num1> <num2>
            ");
            return;
        }
    };
    
    let val2 = match x.get_arg_at(4){
        Some(value) => check_for_valid_no(value.to_owned()),
        None => {
            x.print_help("Please provide 2 numeric values \n  usage: calc -o <operand> <num1> <num2>
            ");
            return;
        }
    };


    let answer = calculate(operand.as_str(), val1, val2);

    println!("Answer is : {}", answer);

    
}

fn check_for_valid_no(value : String) -> u8 {
    match value.parse::<u8>(){
        Ok(v) => v,
        Err(_) => {
            println!("Please provide a valid number");
            std::process::exit(1);
        }
    }
}


fn calculate (operand : &str, num1 : u8, num2 : u8) -> u8 {
    match operand {
        "a" => num1 + num2,
        "s" => num1 - num2,
        "m" => num1 * num2,
        "d" => num1 / num2,
        "r" => num1 % num2,
        _ => 0,
    }
}