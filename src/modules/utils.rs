//! Hilfsfunktionen für das Projekt

use crate::modules::error::ProjectError;
use std::fs;
use std::path::Path;
use uuid::Uuid;

/// Formatiert eine Zahl mit einer bestimmten Genauigkeit
pub fn format_number(number: f64, precision: usize) -> String {
    format!("{:.precision$}", number, precision = precision)
}

/// Generiert eine eindeutige ID
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// Liest eine Datei und gibt den Inhalt zurück
pub fn read_file_content(file_path: &str) -> Result<String, ProjectError> {
    fs::read_to_string(file_path).map_err(ProjectError::from)
}

/// Schreibt Inhalt in eine Datei
pub fn write_file_content(file_path: &str, content: &str) -> Result<(), ProjectError> {
    fs::write(file_path, content).map_err(ProjectError::from)
}

/// Prüft, ob eine Datei existiert
pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

/// Erstellt ein Verzeichnis, falls es nicht existiert
pub fn create_directory(dir_path: &str) -> Result<(), ProjectError> {
    if !Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path).map_err(ProjectError::from)?;
    }
    Ok(())
}

/// Validiert eine Email-Adresse (einfache Überprüfung)
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

/// Berechnet den Durchschnitt einer Liste von Zahlen
pub fn calculate_average(numbers: &[f64]) -> Result<f64, ProjectError> {
    if numbers.is_empty() {
        return Err(ProjectError::InvalidInput("Liste ist leer".to_string()));
    }
    
    let sum: f64 = numbers.iter().sum();
    Ok(sum / numbers.len() as f64)
}

/// Findet das Minimum in einer Liste von Zahlen
pub fn find_minimum(numbers: &[f64]) -> Result<f64, ProjectError> {
    if numbers.is_empty() {
        return Err(ProjectError::InvalidInput("Liste ist leer".to_string()));
    }
    
    Ok(numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b)))
}

/// Findet das Maximum in einer Liste von Zahlen
pub fn find_maximum(numbers: &[f64]) -> Result<f64, ProjectError> {
    if numbers.is_empty() {
        return Err(ProjectError::InvalidInput("Liste ist leer".to_string()));
    }
    
    Ok(numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)))
}

/// Sortiert eine Liste von Zahlen
pub fn sort_numbers(numbers: &mut [f64]) {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
}

/// Berechnet die Standardabweichung
pub fn calculate_standard_deviation(numbers: &[f64]) -> Result<f64, ProjectError> {
    if numbers.is_empty() {
        return Err(ProjectError::InvalidInput("Liste ist leer".to_string()));
    }
    
    let mean = calculate_average(numbers)?;
    let variance = numbers.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / numbers.len() as f64;
    
    Ok(variance.sqrt())
}

/// Konvertiert Celsius zu Fahrenheit
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Konvertiert Fahrenheit zu Celsius
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(3.14159, 2), "3.14");
        assert_eq!(format_number(10.0, 0), "10");
        assert_eq!(format_number(1.23456, 4), "1.2346");
    }

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();
        
        assert_ne!(id1, id2);
        assert!(id1.len() > 0);
        assert!(id2.len() > 0);
    }

    #[test]
    fn test_file_operations() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();
        
        // Test schreiben
        let content = "Test content";
        write_file_content(file_path, content).unwrap();
        
        // Test lesen
        let read_content = read_file_content(file_path).unwrap();
        assert_eq!(read_content, content);
        
        // Test existiert
        assert!(file_exists(file_path));
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(validate_email("user@domain.org"));
        assert!(!validate_email("invalid-email"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("user@"));
    }

    #[test]
    fn test_calculate_average() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let avg = calculate_average(&numbers).unwrap();
        assert_eq!(avg, 3.0);
        
        let empty: Vec<f64> = vec![];
        assert!(calculate_average(&empty).is_err());
    }

    #[test]
    fn test_find_minimum() {
        let numbers = vec![3.0, 1.0, 4.0, 1.5, 9.0];
        let min = find_minimum(&numbers).unwrap();
        assert_eq!(min, 1.0);
        
        let empty: Vec<f64> = vec![];
        assert!(find_minimum(&empty).is_err());
    }

    #[test]
    fn test_find_maximum() {
        let numbers = vec![3.0, 1.0, 4.0, 1.5, 9.0];
        let max = find_maximum(&numbers).unwrap();
        assert_eq!(max, 9.0);
        
        let empty: Vec<f64> = vec![];
        assert!(find_maximum(&empty).is_err());
    }

    #[test]
    fn test_sort_numbers() {
        let mut numbers = vec![3.0, 1.0, 4.0, 1.5, 9.0];
        sort_numbers(&mut numbers);
        assert_eq!(numbers, vec![1.0, 1.5, 3.0, 4.0, 9.0]);
    }

    #[test]
    fn test_calculate_standard_deviation() {
        let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std_dev = calculate_standard_deviation(&numbers).unwrap();
        assert!((std_dev - 2.0).abs() < 0.1);
        
        let empty: Vec<f64> = vec![];
        assert!(calculate_standard_deviation(&empty).is_err());
    }

    #[test]
    fn test_temperature_conversion() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn test_create_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let new_dir = temp_dir.path().join("new_directory");
        let new_dir_str = new_dir.to_str().unwrap();
        
        create_directory(new_dir_str).unwrap();
        assert!(file_exists(new_dir_str));
    }
}
