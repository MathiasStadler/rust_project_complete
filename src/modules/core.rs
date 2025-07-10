//! Kernfunktionalität des Projekts

use crate::modules::error::ProjectError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Konfiguration für das Projekt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub max_history_size: usize,
    pub precision: u32,
    pub debug_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_history_size: 1000,
            precision: 2,
            debug_mode: false,
        }
    }
}

/// Statistiken für Operationen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub total_operations: u64,
    pub operation_counts: HashMap<String, u64>,
    pub average_result: f64,
    pub min_result: f64,
    pub max_result: f64,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            total_operations: 0,
            operation_counts: HashMap::new(),
            average_result: 0.0,
            min_result: f64::MAX,
            max_result: f64::MIN,
        }
    }
}

impl Statistics {
    /// Erstellt neue Statistiken
    pub fn new() -> Self {
        Self::default()
    }

    /// Fügt eine Operation zu den Statistiken hinzu
    pub fn add_operation(&mut self, operation_type: &str, result: f64) {
        self.total_operations += 1;
        
        *self.operation_counts.entry(operation_type.to_string()).or_insert(0) += 1;
        
        if self.total_operations == 1 {
            self.average_result = result;
            self.min_result = result;
            self.max_result = result;
        } else {
            // Berechne neuen Durchschnitt
            let old_sum = self.average_result * (self.total_operations - 1) as f64;
            self.average_result = (old_sum + result) / self.total_operations as f64;
            
            // Aktualisiere Min/Max
            if result < self.min_result {
                self.min_result = result;
            }
            if result > self.max_result {
                self.max_result = result;
            }
        }
    }

    /// Gibt die Anzahl der Operationen eines bestimmten Typs zurück
    pub fn get_operation_count(&self, operation_type: &str) -> u64 {
        self.operation_counts.get(operation_type).copied().unwrap_or(0)
    }

    /// Prüft, ob Statistiken leer sind
    pub fn is_empty(&self) -> bool {
        self.total_operations == 0
    }

    /// Löscht alle Statistiken
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// Mathematische Hilfsfunktionen
pub struct MathUtils;

impl MathUtils {
    /// Berechnet den größten gemeinsamen Teiler
    pub fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    /// Berechnet das kleinste gemeinsame Vielfache
    pub fn lcm(a: u64, b: u64) -> Result<u64, ProjectError> {
        if a == 0 || b == 0 {
            return Err(ProjectError::InvalidInput("LCM von Null ist nicht definiert".to_string()));
        }
        
        let gcd = Self::gcd(a, b);
        let result = (a / gcd).checked_mul(b)
            .ok_or_else(|| ProjectError::OverflowError("LCM Overflow".to_string()))?;
        
        Ok(result)
    }

    /// Prüft, ob eine Zahl eine Primzahl ist
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        
        let sqrt_n = (n as f64).sqrt() as u64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    /// Berechnet die n-te Fibonacci-Zahl
    /* pub fn _org_fibonacci(n: u64) -> Result<u64, ProjectError> {
        if n > 93 {
            return Err(ProjectError::OverflowError("Fibonacci Overflow".to_string()));
        }
        
        if n <= 1 {
            return Ok(n);
        }
        
        let mut a = 0;
        let mut b = 1;
        
        for _ in 2..=n {
            let temp = a.checked_add(b)
                .ok_or_else(|| ProjectError::OverflowError("Fibonacci Overflow".to_string()))?;
            a = b;
            b = temp;
        }
        
        Ok(b)
    } */

    /// Berechnet die n-te Fibonacci-Zahl
pub fn fibonacci(n: u64) -> Result<u64, ProjectError> {
    if n > 93 {
        return Err(ProjectError::OverflowError("Fibonacci Overflow".to_string()));
    }
    
    if n <= 1 {
        return Ok(n);
    }
    
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    
    for _ in 2..=n {
        let temp = a.checked_add(b)
            .ok_or_else(|| ProjectError::OverflowError("Fibonacci Overflow".to_string()))?;
        a = b;
        b = temp;
    }
    
    Ok(b)
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.max_history_size, 1000);
        assert_eq!(config.precision, 2);
        assert!(!config.debug_mode);
    }

    #[test]
    fn test_statistics_new() {
        let stats = Statistics::new();
        assert_eq!(stats.total_operations, 0);
        assert!(stats.is_empty());
    }

    #[test]
    fn test_statistics_add_operation() {
        let mut stats = Statistics::new();
        stats.add_operation("add", 5.0);
        
        assert_eq!(stats.total_operations, 1);
        assert_eq!(stats.get_operation_count("add"), 1);
        assert_eq!(stats.average_result, 5.0);
        assert_eq!(stats.min_result, 5.0);
        assert_eq!(stats.max_result, 5.0);
    }

    #[test]
    fn test_statistics_multiple_operations() {
        let mut stats = Statistics::new();
        stats.add_operation("add", 2.0);
        stats.add_operation("add", 8.0);
        stats.add_operation("multiply", 10.0);
        
        assert_eq!(stats.total_operations, 3);
        assert_eq!(stats.get_operation_count("add"), 2);
        assert_eq!(stats.get_operation_count("multiply"), 1);
        assert_eq!(stats.average_result, (2.0 + 8.0 + 10.0) / 3.0);
        assert_eq!(stats.min_result, 2.0);
        assert_eq!(stats.max_result, 10.0);
    }

    #[test]
    fn test_statistics_clear() {
        let mut stats = Statistics::new();
        stats.add_operation("add", 5.0);
        stats.clear();
        
        assert!(stats.is_empty());
        assert_eq!(stats.total_operations, 0);
    }

    #[rstest]
    #[case(12, 8, 4)]
    #[case(54, 24, 6)]
    #[case(17, 13, 1)]
    #[case(0, 5, 5)]
    fn test_gcd(#[case] a: u64, #[case] b: u64, #[case] expected: u64) {
        assert_eq!(MathUtils::gcd(a, b), expected);
    }

    #[rstest]
    #[case(12, 8, 24)]
    #[case(4, 6, 12)]
    #[case(7, 13, 91)]
    fn test_lcm(#[case] a: u64, #[case] b: u64, #[case] expected: u64) {
        assert_eq!(MathUtils::lcm(a, b).unwrap(), expected);
    }

    #[test]
    fn test_lcm_with_zero() {
        assert!(MathUtils::lcm(0, 5).is_err());
        assert!(MathUtils::lcm(5, 0).is_err());
    }

    #[rstest]
    #[case(2, true)]
    #[case(3, true)]
    #[case(17, true)]
    #[case(97, true)]
    #[case(1, false)]
    #[case(4, false)]
    #[case(15, false)]
    #[case(100, false)]
    fn test_is_prime(#[case] n: u64, #[case] expected: bool) {
        assert_eq!(MathUtils::is_prime(n), expected);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(10, 55)]
    #[case(20, 6765)]
    fn test_fibonacci(#[case] n: u64, #[case] expected: u64) {
        assert_eq!(MathUtils::fibonacci(n).unwrap(), expected);
    }

    #[test]
    fn test_fibonacci_overflow() {
        assert!(MathUtils::fibonacci(100).is_err());
    }
}
