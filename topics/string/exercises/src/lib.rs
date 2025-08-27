pub fn hello() -> String {
    // return "Hello Rust".to_string();
    return String::from("Hello Rust");
}

pub fn greet(name: &str) -> String {
    return format!("Hello {name}");
}

pub fn append(mut s: String) -> String {
    return format!("{s}!");
}
