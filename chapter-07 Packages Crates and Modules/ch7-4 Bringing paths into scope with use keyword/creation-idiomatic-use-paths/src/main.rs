use std::collections::HashMap;

fn main() {
    let mut map: HashMap<u8, String> = HashMap::new();
    map.insert(1, String::from("One"));
    map.insert(2, String::from("Two"));
    println!("The map: {map:?}")
}
