fn main() {
    let s = String::from("hello world");

    println!("{}", first_word(&s));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
