### WHATS NEW


##### Change `fli.get_values(n)` method type from `Result<Option<Vec[<String]>>` to `Result<Vec<String>, &str>`

> example

```diff
 fn main(){
    let mut app : Fli = Fli::init("my app");
    app.option("-n --name, <>", |x : &Fli| {});
    app.option("-g --greet", |x : &Fli| {
        match x.get_values("--name".to_owned()){
-            Ok(option) => {
-                if let Some(v) = option{ println!("Good day {?}", v[0]);}
+           Ok(values) => {
+              println!("Good day {?}", values[0]);}
            },
            Err(_) => {},
        }
    });
    // required for the app  to work 
    app.run();
}
```
#### Description needed in `init`, `command` , `option` method

```diff
- let mut app = Fli::init("app-name");
- let moveCommand = app.command("move");
- moveCommand.option("-p --path, <...>", move_file);
- app.option("-g --greet", greet);

+ let mut app = Fli::init("app-name", "app-description");
+ let moveCommand = app.command("move", "move files");
+ moveCommand.option("-p --path, <...>", "path to files to be moved", move_file);
+ app.option("-g --greet",  "to make a greeting", greet);
```

#### Can now print help screen and also end process
```rust
let mut app : Fli = Fli::init("my app", "my app description");
app.option("-n --name, <>", "Name to call you", |x : &Fli| {});
app.option("-g --greet", "greeting", |x : &Fli| {
    match x.get_values("name".to_owned() /* passing (--name, -n or n) would work*/){
        Ok(value) => {
            println!("Good day {?}", value[0])
        },
        Err(e) => {
            x.print_help("No name found"+e.to_str()) // this ends the process
        },
    }
});
```