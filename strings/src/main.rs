fn main() {
    let text = "Hello\n!";
    let upper = text.to_ascii_uppercase();
    let stripped = upper.strip_prefix("HELLO\N").unwrap();
    println!("{}", first_line(stripped));
}

pub fn first_line(string: String) -> String {
    string.lines().next().unwrap()
}
