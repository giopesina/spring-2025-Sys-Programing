#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    *total = (low..=high).sum();
}

fn main() {
    let mut total = 0;
    let low = 0;
    let high = 100;
    
    sum(&mut total, low, high);
    
    println!("Total sum from {} to {} is: {}", low, high, total); // Should print: Total sum from 0 to 100 is: 5050
}
