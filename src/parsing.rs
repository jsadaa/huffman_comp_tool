use std::collections::HashMap;

pub(crate) fn count_bytes(s: &str) -> HashMap<u8, i32> {
    let mut map: HashMap<u8, i32> = HashMap::new();
    for c in s.bytes() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }
    map
}