fn main() {
    let text = "the first apple fell off the tree über gänse";

    println!("Original:  {}", text);
    println!("Pig latin: {}", pig_latin(&text));
}

fn pig_latin(text: &str) -> String {
    let words: Vec<_> = text.to_string()
                            .split_whitespace()
                            .map(|word| pig_latin_word(word))
                            .collect();
    words.join(" ")
}

fn pig_latin_word(word: &str) -> String {
    const VOWELS: [char; 8] = ['a', 'e', 'i', 'o', 'u', 'ä', 'ö', 'ü'];
    let mut iterator = word.chars();
    let first_char = iterator.next().unwrap();
    if VOWELS.contains(&first_char) {
        word.to_string() + "-hay"
    } else {
        let remaining_word: String = iterator.collect();
        remaining_word + "-" + &first_char.to_string() + "ay"
    }
}
