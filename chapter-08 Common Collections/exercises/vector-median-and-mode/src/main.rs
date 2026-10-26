use std::collections::HashMap;

use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let mut random_vec: Vec<i32> = (0..10).map(|_| rng.random_range(0..100)).collect();
    // Add 30 as mode
    random_vec.push(30);
    random_vec.push(30);
    random_vec.push(30);

    println!("unsorted: {:?}", random_vec);

    // sort the vector
    random_vec.sort();
    println!("sorted: {:?}", random_vec);

    let mid_index = random_vec.len() / 2;
    let median = random_vec.get(mid_index);

    println!("The median is: {median:?}");

    // Find the mode
    let mut numbers_hash: HashMap<i32, i32> = HashMap::new();

    for number in random_vec {
        let count = numbers_hash.entry(number).or_insert(0);
        *count += 1;
    }

    println!("Numbers HashMap: {numbers_hash:?}");

    let mode = numbers_hash.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num);
    println!("The mode is: {mode:?}");
}
