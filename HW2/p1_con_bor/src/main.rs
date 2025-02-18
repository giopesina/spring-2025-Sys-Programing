fn concat_strings(s1: &String, s2: &String) -> String {
    let mut s = String::from(s1);
    s.push_str(s2);
    return s;
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}