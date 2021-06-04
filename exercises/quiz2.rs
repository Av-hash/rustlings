// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // literal
    string("red".to_string()); // to_string converts a literal to string
    string(String::from("hi")); // literal to string
    string("rust is fun!".to_owned()); // to_owned converts a &str to String
    string("nice weather".into()); // just like from
    string(format!("Interpolation {}", "Station")); // format! gives String
    string_slice(&String::from("abc")[0..1]); // slice
    string_slice("  hello there ".trim()); // returns the same string type
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // to_string converts literal to string and .replace returns the same type
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // returns String type
}
