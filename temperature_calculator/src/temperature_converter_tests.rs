#[cfg(test)]
mod temperature_converter_tests {
    use crate::temperature_convertor;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let celsius = 0.0;
        let fahrenheit = temperature_convertor::celsius_to_fahrenheit(celsius);
        assert_eq!(fahrenheit, 32.0);
    }

    #[test]
    fn test_farhenheit_to_celsius() {
        let fahrenheit = 32.0;
        let celsius = temperature_convertor::fahrenheit_to_celsius(fahrenheit);
        assert_eq!(celsius, 0.0);
    }

}

fn main() {}