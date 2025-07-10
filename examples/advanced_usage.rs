//! Erweiterte Verwendung mit Statistiken und Utils

use rust_project_complete::modules::core::{Statistics, MathUtils};
use rust_project_complete::modules::utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Erweiterte Funktionen");
    println!("====================");
    
    // Statistiken
    let mut stats = Statistics::new();
    stats.add_operation("add", 10.0);
    stats.add_operation("multiply", 20.0);
    stats.add_operation("subtract", 5.0);
    
    println!("Statistiken:");
    println!("Total: {}", stats.total_operations);
    println!("Durchschnitt: {:.2}", stats.average_result);
    println!("Min: {:.2}", stats.min_result);
    println!("Max: {:.2}", stats.max_result);
    
    // Mathematische Utilities
    println!("\nMathematische Funktionen:");
    println!("GCD(48, 18) = {}", MathUtils::gcd(48, 18));
    println!("LCM(48, 18) = {}", MathUtils::lcm(48, 18)?);
    println!("Is 17 prime? {}", MathUtils::is_prime(17));
    println!("Fibonacci(10) = {}", MathUtils::fibonacci(10)?);
    
    // Utility-Funktionen
    println!("\nUtility-Funktionen:");
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("Zahlen: {:?}", numbers);
    println!("Durchschnitt: {:.2}", calculate_average(&numbers)?);
    println!("Minimum: {:.2}", find_minimum(&numbers)?);
    println!("Maximum: {:.2}", find_maximum(&numbers)?);
    println!("Standardabweichung: {:.2}", calculate_standard_deviation(&numbers)?);
    
    // Temperaturkonvertierung
    println!("\nTemperaturkonvertierung:");
    println!("0°C = {:.1}°F", celsius_to_fahrenheit(0.0));
    println!("32°F = {:.1}°C", fahrenheit_to_celsius(32.0));
    
    Ok(())
}
