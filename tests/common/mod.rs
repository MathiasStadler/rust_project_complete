//! Gemeinsame Test-Utilities

use rust_project_complete::Calculator;

pub fn setup_calculator_with_history() -> Calculator {
    let mut calc = Calculator::new();
    calc.add(1.0, 2.0);
    calc.multiply(3.0, 4.0);
    calc.subtract(10.0, 5.0);
    calc
}

pub fn assert_float_eq(a: f64, b: f64, epsilon: f64) {
    assert!((a - b).abs() < epsilon, "Expected {}, got {}", a, b);
}

pub fn generate_test_numbers(count: usize) -> Vec<f64> {
    (1..=count).map(|i| i as f64).collect()
}
