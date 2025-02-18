fn clone_and_modify(s: &String) -> String {
    let mut str11 = s.clone();
    str11.push_str("World!");
    return str11;
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}