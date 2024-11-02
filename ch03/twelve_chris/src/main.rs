const NUMERALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth",
    "eleventh", "twelvth",
];

const LINES: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn main() {
    for stanza_index in 0..12 {
        print_head(stanza_index);
        print_body(stanza_index);
        println!();
    }
}

fn print_head(stanza_index: usize) {
    println!(
        "On the {} day of Christmas my true love sent to me",
        NUMERALS[stanza_index]
    );
}

fn print_body(stanza_index: usize) {
    for line_index in (0..=stanza_index).rev() {
        println!("{}", LINES[line_index]);
    }
}
