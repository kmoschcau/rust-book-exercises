fn main() {
    const ITER_TARGET: u8 = 150;

    const F0: u8 = 0;
    const F1: u8 = 1;

    let mut last: u128 = F0.into();
    let mut current: u128 = F1.into();

    for _ in 1..=(ITER_TARGET - 2) {
        let new = last + current;
        last = current;
        current = new;
    }

    println!("{}. fibonacci number: {}", ITER_TARGET, current);
}
