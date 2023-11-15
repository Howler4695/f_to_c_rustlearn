use std::io;

fn main() {
    loop {
        println!("Farenheit to Celcius (C) or Celcius to Farenheit (F)?");

        let mut conversion_type = String::new();

        io::stdin().read_line(&mut conversion_type).expect("Failed to Read line.");

        let conversion_type: char = match conversion_type.trim().to_uppercase().parse() {
            Ok(character) => character,
            Err(_) => {
                println!("Please type a character.\n");
                continue;
            }
        };

        let mut conversion_coefficient: f64 = 5f64/9f64;

        let mut conversion_add_subtract: f64 = -32.0;

        match conversion_type {
            'C' => {
                println!("Farenheit to Celcius");
            },
            'F' => {
                println!("Celcius to Farenheit");
                conversion_coefficient =  9f64 / 5f64;
                conversion_add_subtract = 32.0;
            },
            _ => {
                println!("Character Not detected, please type C or F");
                continue;
            }
        }

        println!("-------------------\nPlease enter temp: ");

        let mut input_value = String::new();

        io::stdin().read_line(&mut input_value).expect("Failed to Read line.");

        let input_value: f64 = match input_value.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("Please type a number.\n");
                continue;
            }
        };

        let converted_value: f64 = (&input_value + &conversion_add_subtract) * &conversion_coefficient;

        println!("Converted Value: {:.1}°{}\n-------------------", converted_value, conversion_type);
    }
}

 