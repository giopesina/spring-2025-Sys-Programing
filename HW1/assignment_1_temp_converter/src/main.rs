const FREEZE_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let conv = 5.0 / 9.0 * (f - FREEZE_POINT);
    conv
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let conv = c * (9.0 / 5.0) + FREEZE_POINT;
    conv
}

fn main() {
    let mut tempf = 92.7;
    for x in 0..=5 {
        println!("{:.2} degrees C", fahrenheit_to_celsius(tempf));
        tempf += 1.0;
    }
}
