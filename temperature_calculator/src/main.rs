mod temperature_convertor;
mod temperature_converter_tests;

fn main() {
    println!("Choose an option:");

    println!("1. Convert Celsius to Fahrenheit");
    println!("2. Convert Fahrenheit to Celsius");

    let mut option = String::new();

    std::io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid option");
            return;
        },
    };

    println!("Enter the temperature:");

    let mut temperature = String::new();

    std::io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid option");
            return;
        },
    };

    match option {
        1 => {
            let fahrenheit = temperature_convertor::celsius_to_fahrenheit(temperature);
            println!("{} Celsius is {} Fahrenheit", temperature, fahrenheit);
        },
        2 => {
            let celsius = temperature_convertor::fahrenheit_to_celsius(temperature);
            println!("{} Fahrenheit is {} Celsius", temperature, celsius);
        },
        _ => println!("Invalid option"),
    };
}
