use std::collections::HashMap;

fn main() {
    let strs = vec!["the", "that", "this", "thus"];
    let mut common_map = HashMap::new();

    // Initialize map with counts for each character in all strings
    for s in &strs {
        let mut seen = std::collections::HashSet::new();
        for c in s.chars() {
            seen.insert(c); // avoid double-counting chars in the same word
        }
        for c in seen {
            *common_map.entry(c).or_insert(0) += 1;
        }
    }

    // Get common characters in order from the first string
    let mut printed = std::collections::HashSet::new();
    for c in strs[0].chars() {
        if !printed.contains(&c) && common_map.get(&c) == Some(&strs.len()) {
            print!("{}", c);
            printed.insert(c);
        }
    }

    println!();
}
