fn main() {
    another_function(five(), six());
}

fn five() -> i32 {
    5
}
fn six() -> i32 {
    6
}

fn another_function(x: i32, y: i32) {
    println!("The values are:");
    println!("    x: {}", x);
    println!("    y: {}", y);
}
