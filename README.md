# FLI

A library for creating command line apps in rust inspired by the like of [commander.js](https://github.com/tj/commander.js)

> NOTE: Check out [sample code](https://github.com/codad5/fli/blob/master/sample/src/main.rs)

## Changes
For stable changes check the [CHANGELOG.md](https://github.com/codad5/fli/blob/master/CHANGELOG.md) file 

For unstable changes check the [CHANGELOG.md](https://github.com/codad5/fli/blob/dev/CHANGELOG.md) file

```rust
use fli::{Fli, init_fli_from_toml};

fn main(){
    let mut app : Fli = init_fli_from_toml!(); // using the toml file
    app.option("-n --name, <>", "Name to call you", |x : &Fli| {});
    app.option("-g --greet", "greeting", |x : &Fli| {
        match x.get_values("name".to_owned() /* passing (--name, -n or n) would work*/){
            Ok(value) => {
                println!("Good day {?}", value[0])
            },
            Err(_) => {},
        }
    });
    // required for the app  to work 
    app.run();
}
```


then run it like this
```bash
$ cargo run -- -g -n james
Good day james
```

## Getting started
### Installtion
```bash
cargo add fli
```
OR
```toml
[dependencies]
fli = "0.0.5"
```

### Import
```rust
extern crate fli;
use fli::Fli;
fn main(){
    println!("Happy Coding");
}
```
### Create an App Instance 

```rust
fn main(){
    let mut app = init_fli_from_toml!(); // to init from your cargo.toml file
}

```
> OR

```rust
fn main(){
    let mut app = Fli::init("app-name", "an app description");
    app.set_version("0.0.1");
}
```


### Adding Options

```rust
fn main(){
    let mut app = init_fli_from_toml!();
    app.option("-g --greet", "to make a greeting", greet);
    app.option("-n --name, <>", "to set your name", |x|{});
}

// callback for the greet param
fn greet(x: &Fli){
    match x.get_values("--name".to_owned()){
        Ok(option) => {
            println!("Good day {?}", v[0]);
        },
        Err(_) => {},
    }
}
```

### Run your app
```rust
fn main(){
    //... other code
    app.run();
}
```

### Adding a new Command Set
You can also add a new command set using the command method
```rust
fn main(){
    let mut app = init_fli_from_toml!();
    app.command("greet", "An app that respects")
    .default(greet)
    .allow_inital_no_param_values(false)
    .option("-n --name, <>", "To print your name along side", greet)
    .option("-t --time, []", "For time based Greeting", greet);
    app.run();
}

fn greet(x){ /*code to greet "*/ }
```
Then you would run the command like this
```shell
$ cargo run -- greet -n "codad5" 
> Hello Codad5
```

### Doing it in a procedual way
Like commander.js you can also do it in a procedual way

```rust
use fli::Fli;

fn main(){
    //  doing it procedual way
    let mut app = init_fli_from_toml!();
    let moveCommand = app.command("move", "move files");
    // the [...] means accept optional multiple
    moveCommand.option("-p --path, <...>", "path to files to be moved", move_file);
    app.option("-g --greet",  "to make a greeting", |x|{});
    app.option("-n --name, <>", "to set your name", |x|{});
    app.run();

    if app.is_passed("--greet"){
        if app.has_a_value("-n"){/* greet person with name*/}
        else { /* freet without name*/ }
    }
}
```

## Explaining all `app : Fli` methods
**Note:**
> All `app : Fli` methods are avaliable as `app : &Fli` methods
- `app.option(arg_and_data, callback)` : 
This method takes in 2 param 
  - First `arg_and_data` : This is a format template of how the avaliable arguments for a command would be being in a format `-a --arg` or `-a --arg, data` where `-a` is the short  form of the argument and `--arg` is the long form of the argument. `--data` is the acceptable data type and it is seperated by a **comma** `,` , if not passed then the arg does not need a data type
  
  | symbol | meaning |
  |:---:|:---|
  | [] | This means it needs one optional data|
  | <> | This means it needs one required data |
  | [...] | This means it can take in many optional data |
  | <...> | This means it needs at least one data, can take more |


- `app.commad(command_name)` : 
This is to create a new command with its own option and param like

- `app.default(callback)` : The default callback incase no args or command is being passed but a `no_param_value` is being passed 

- `app.allow_duplicate_callback(bool)` : To prevent duplicate callbacks as a result of same callback of an `arg` or as a result of the `arg` been passed multiple times
    - True : Turns it on i.e the code below would work fine
    ```bash 
    $ myapp "some ran value"
    ```

- `app.allow_inital_no_param_values(bool)` : This is to allow values to a command with no params  

- `app.run()` **(!important)** : To run the app , 

- `app.has_a_value(arg_flag)` : Check if an arg has a value 
- `app.get_values(arg_flag)` : get the value(s) of  an expect required param,  this returns a `Result` Type with a vector of string as the Ok value `Vec<String>` and `&str` as the error value 
> NOTE  the method `get_values` would return the `Err` Enum if the arg does not expect or require a value

- `app.is_passed(bool)` : Check if an arg flag is passed. 

> NOTE : using the methods `has_a_value`, `get_values` and `is_passed` you can pass `-n , --name , n , name` and they would all return same expected value

- `app.get_arg_at(u8)` : Get Arg at a specific position 
> NOTE :  The runner is not included as part of the arg list . ie if a command like this `my-app > greet > hello` exist the position 1 for the command `greet` is greet and not `my-app`

- `app.print_help(message)` : Prints a well descriptive message.


>Printing default help thisGet the app general help option
> ```shell
> $ my-app --help
> ```
Get the move command help option
```shell
$ my-app move --help # a new command called
```


