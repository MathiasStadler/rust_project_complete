//! Integrationstests

use rust_project_complete::Calculator;
use rust_project_complete::modules::core::{Statistics, MathUtils};
use rust_project_complete::modules::utils::*;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_calculator_integration() {
    let mut calc = Calculator::new();
    
    // Komplexe Berechnungssequenz
    let result1 = calc.add(10.0, 5.0);
    let result2 = calc.multiply(result1, 2.0);
    let result3 = calc.subtract(result2, 5.0);
    let result4 = calc.divide(result3, 5.0).unwrap();
    
    assert_eq!(result4, 5.0);
    assert_eq!(calc.history_count(), 4);
}

#[test]
fn test_statistics_integration() {
    let mut stats = Statistics::new();
    
    // Füge verschiedene Operationen hinzu
    stats.add_operation("add", 10.0);
    stats.add_operation("multiply", 20.0);
    stats.add_operation("subtract", 5.0);
    stats.add_operation("add", 15.0);
    
    assert_eq!(stats.total_operations, 4);
    assert_eq!(stats.get_operation_count("add"), 2);
    assert_eq!(stats.get_operation_count("multiply"), 1);
    assert_eq!(stats.average_result, 12.5);
    assert_eq!(stats.min_result, 5.0);
    assert_eq!(stats.max_result, 20.0);
}

#[test]
fn test_math_utils_integration() {
    // Teste mehrere mathematische Funktionen zusammen
    let a = 12u64;
    let b = 8u64;
    
    let gcd = MathUtils::gcd(a, b);
    let lcm = MathUtils::lcm(a, b).unwrap();
    
    assert_eq!(gcd, 4);
    assert_eq!(lcm, 24);
    
    // Teste Primzahlen
    assert!(MathUtils::is_prime(17));
    assert!(!MathUtils::is_prime(18));
    
    // Teste Fibonacci
    assert_eq!(MathUtils::fibonacci(10).unwrap(), 55);
}

#[test]
fn test_file_operations_integration() {
    let mut temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();
    
    // Schreibe JSON-Daten
    let calc = Calculator::new();
    let json_data = serde_json::to_string_pretty(&calc).unwrap();
    
    write_file_content(file_path, &json_data).unwrap();
    
    // Lese und parse JSON
    let read_data = read_file_content(file_path).unwrap();
    let parsed_calc: Calculator = serde_json::from_str(&read_data).unwrap();
    
    assert_eq!(parsed_calc.history_count(), 0);
}

#[test]
fn test_utils_integration() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    
    let avg = calculate_average(&numbers).unwrap();
    let min = find_minimum(&numbers).unwrap();
    let max = find_maximum(&numbers).unwrap();
    let std_dev = calculate_standard_deviation(&numbers).unwrap();
    
    assert_eq!(avg, 3.0);
    assert_eq!(min, 1.0);
    assert_eq!(max, 5.0);
    assert!((std_dev - 1.5811).abs() < 0.001);
}

#[test]
fn test_error_handling_integration() {
    let mut calc = Calculator::new();
    
    // Teste Division durch Null
    let result = calc.divide(10.0, 0.0);
    assert!(result.is_err());
    
    // Teste Fakultät mit zu großer Zahl
    let result = calc.factorial(100);
    assert!(result.is_err());
    
    // Teste LCM mit Null
    let result = MathUtils::lcm(0, 5);
    assert!(result.is_err());
}

#[test]
fn test_serialization_integration() {
    let mut calc = Calculator::new();
    calc.add(1.0, 2.0);
    calc.multiply(3.0, 4.0);
    
    // Serialisiere zu JSON
    let json = serde_json::to_string(&calc).unwrap();
    
    // Deserialisiere von JSON
    let deserialized: Calculator = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.history_count(), 2);
    assert_eq!(deserialized.get_history().len(), 2);
}

#[test]
fn test_concurrent_operations() {
    use std::thread;
    use std::sync::{Arc, Mutex};
    
    let calc = Arc::new(Mutex::new(Calculator::new()));
    let mut handles = vec![];
    
    // Erstelle mehrere Threads für parallele Berechnungen
    for i in 0..5 {
        let calc_clone = Arc::clone(&calc);
        let handle = thread::spawn(move || {
            let mut calc = calc_clone.lock().unwrap();
            calc.add(i as f64, i as f64);
        });
        handles.push(handle);
    }
    
    // Warte auf alle Threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_calc = calc.lock().unwrap();
    assert_eq!(final_calc.history_count(), 5);
}
