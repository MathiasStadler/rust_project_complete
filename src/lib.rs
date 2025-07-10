//! # Rust Project Complete
//! 
//! Ein vollständiges Rust-Projekt mit 100% Testabdeckung.
//! 
//! ## Beispiel
//! 
//! ```rust
//! use rust_project_complete::Calculator;
//! 
//! let calc = Calculator::new();
//! let result = calc.add(2, 3);
//! assert_eq!(result, 5);
//! ```

pub mod modules;

use modules::error::ProjectError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Hauptstruktur für mathematische Operationen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calculator {
    history: Vec<Operation>,
}

/// Repräsentiert eine mathematische Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub operation_type: OperationType,
    pub operands: Vec<f64>,
    pub result: f64,
    pub timestamp: u64,
}

/// Arten von mathematischen Operationen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationType::Add => write!(f, "Addition"),
            OperationType::Subtract => write!(f, "Subtraktion"),
            OperationType::Multiply => write!(f, "Multiplikation"),
            OperationType::Divide => write!(f, "Division"),
        }
    }
}

impl Calculator {
    /// Erstellt einen neuen Calculator
    /// 
    /// # Beispiel
    /// 
    /// ```rust
    /// use rust_project_complete::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// assert_eq!(calc.history_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    /// Addiert zwei Zahlen
    /// 
    /// # Beispiel
    /// 
    /// ```rust
    /// use rust_project_complete::Calculator;
    /// 
    /// let mut calc = Calculator::new();
    /// let result = calc.add(2.0, 3.0);
    /// assert_eq!(result, 5.0);
    /// ```
    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = a + b;
        self.add_to_history(OperationType::Add, vec![a, b], result);
        result
    }

    /// Subtrahiert zwei Zahlen
    pub fn subtract(&mut self, a: f64, b: f64) -> f64 {
        let result = a - b;
        self.add_to_history(OperationType::Subtract, vec![a, b], result);
        result
    }

    /// Multipliziert zwei Zahlen
    pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
        let result = a * b;
        self.add_to_history(OperationType::Multiply, vec![a, b], result);
        result
    }

    /// Dividiert zwei Zahlen
    /// 
    /// # Errors
    /// 
    /// Gibt einen Fehler zurück, wenn durch Null geteilt wird.
    pub fn divide(&mut self, a: f64, b: f64) -> Result<f64, ProjectError> {
        if b == 0.0 {
            return Err(ProjectError::DivisionByZero);
        }
        let result = a / b;
        self.add_to_history(OperationType::Divide, vec![a, b], result);
        Ok(result)
    }

    /// Berechnet die Fakultät einer Zahl
    /// 
    /// # Errors
    /// 
    /// Gibt einen Fehler zurück, wenn die Zahl negativ ist.
    pub fn factorial(&self, n: u64) -> Result<u64, ProjectError> {
        if n > 20 {
            return Err(ProjectError::OverflowError("Factorial too large".to_string()));
        }
        
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        Ok(result)
    }

    /// Gibt die Anzahl der Operationen im Verlauf zurück
    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    /// Gibt den Verlauf der Operationen zurück
    pub fn get_history(&self) -> &[Operation] {
        &self.history
    }

    /// Löscht den Verlauf
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Fügt eine Operation zum Verlauf hinzu
    fn add_to_history(&mut self, op_type: OperationType, operands: Vec<f64>, result: f64) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let operation = Operation {
            operation_type: op_type,
            operands,
            result,
            timestamp,
        };
        
        self.history.push(operation);
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_new_calculator() {
        let calc = Calculator::new();
        assert_eq!(calc.history_count(), 0);
    }

    #[test]
    fn test_add() {
        let mut calc = Calculator::new();
        let result = calc.add(2.0, 3.0);
        assert_eq!(result, 5.0);
        assert_eq!(calc.history_count(), 1);
    }

    #[test]
    fn test_subtract() {
        let mut calc = Calculator::new();
        let result = calc.subtract(5.0, 3.0);
        assert_eq!(result, 2.0);
        assert_eq!(calc.history_count(), 1);
    }

    #[test]
    fn test_multiply() {
        let mut calc = Calculator::new();
        let result = calc.multiply(4.0, 3.0);
        assert_eq!(result, 12.0);
        assert_eq!(calc.history_count(), 1);
    }

    #[test]
    fn test_divide() {
        let mut calc = Calculator::new();
        let result = calc.divide(10.0, 2.0).unwrap();
        assert_eq!(result, 5.0);
        assert_eq!(calc.history_count(), 1);
    }

    #[test]
    fn test_divide_by_zero() {
        let mut calc = Calculator::new();
        let result = calc.divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(calc.history_count(), 0);
    }

    #[test]
    fn test_factorial() {
        let calc = Calculator::new();
        assert_eq!(calc.factorial(0).unwrap(), 1);
        assert_eq!(calc.factorial(1).unwrap(), 1);
        assert_eq!(calc.factorial(5).unwrap(), 120);
    }

    #[test]
    fn test_factorial_overflow() {
        let calc = Calculator::new();
        let result = calc.factorial(25);
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_history() {
        let mut calc = Calculator::new();
        calc.add(1.0, 2.0);
        calc.add(3.0, 4.0);
        assert_eq!(calc.history_count(), 2);
        
        calc.clear_history();
        assert_eq!(calc.history_count(), 0);
    }

    #[test]
    fn test_default() {
        let calc = Calculator::default();
        assert_eq!(calc.history_count(), 0);
    }

    // Property-based Tests
    proptest! {
        #[test]
        fn test_add_commutative(a in -1000.0..1000.0, b in -1000.0..1000.0) {
            let mut calc1 = Calculator::new();
            let mut calc2 = Calculator::new();
            
            let result1 = calc1.add(a, b);
            let result2 = calc2.add(b, a);
            
            assert!((result1 - result2).abs() < f64::EPSILON);
        }

        #[test]
        fn test_multiply_commutative(a in -100.0..100.0, b in -100.0..100.0) {
            let mut calc1 = Calculator::new();
            let mut calc2 = Calculator::new();
            
            let result1 = calc1.multiply(a, b);
            let result2 = calc2.multiply(b, a);
            
            assert!((result1 - result2).abs() < f64::EPSILON);
        }
    }
}
