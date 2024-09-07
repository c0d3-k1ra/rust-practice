pub fn celsius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 1.8 + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}