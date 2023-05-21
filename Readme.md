# FLI

A library for creating commnad line apps in rust

```rust
let mut app : &Fli = Fli::init();
app.option("-n --name", |x : &Fli| {
    println!("say Hello");
});
app.option("-h --hello", |x : &Fli| {
    println!("say Hello");
});
app.run();
```