use crate::fli::Fli;


// The commit messaege should in this format "fli-[<commit-reason(feature, bug, etc)>] [<commit-message>]"
// Example: fli-[feature] [add a new feature]
fn main() {
    let app = Fli::init("sample", "A sample app");
    println!("Hello, world!");
}
