use std::collections::HashMap;

fn main() {
    let vector = vec![12, 5342, 12, 435, 76, 9, 24, 67, 2, 31, 3, 2, 1, 31, 2];

    println!("Vector mean:   {}", mean(&vector));
    println!("Vector median: {}", median(&vector));
    println!("Vector mode:   {}", mode(&vector));
}

fn mean(slice: &[i32]) -> i32 {
    let sum: i32 = slice.iter().sum();
    sum / slice.len() as i32
}

fn median(slice: &[i32]) -> i32 {
    let index = slice.len() / 2;
    let mut sorted = slice.to_vec();
    sorted.sort();
    sorted[index]
}

fn mode(slice: &[i32]) -> i32 {
    let mut counter = HashMap::new();
    for num in slice.iter() {
        *counter.entry(num).or_insert(0) += 1;
    }
    let mut count_vec: Vec<_> = counter.into_iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    *count_vec[0].0
}
