use std::collections::HashMap;

fn main() {
    let mut map: HashMap<u8, &str> = HashMap::with_capacity(30 as usize);
    map.insert(1, "asdf");
    map.insert(2, "2022-08-10");
    map.insert(4, "2022-08-10");
    map.insert(5, "2022-08-10");
    map.insert(6, "2022-08-10");
    let mut min = 255;
    // into_iter会转移所有权，所以这里copy
    for (k, _) in map.clone() {
        if k < min {
            min = k;
        }
    }
    println!("{:?}", min);
    println!("{:?}", map);
    println!("{:?}", map.capacity());
    println!("{:?}", map.len());
}
