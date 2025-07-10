//! Fehlerbehandlung für das Projekt

use thiserror::Error;

/// Projektspezifische Fehler
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProjectError {
    #[error("Division durch Null ist nicht erlaubt")]
    DivisionByZero,
    
    #[error("Overflow-Fehler: {0}")]
    OverflowError(String),
    
    #[error("Ungültige Eingabe: {0}")]
    InvalidInput(String),
    
    #[error("IO-Fehler: {0}")]
    IoError(String),
    
    #[error("Parsing-Fehler: {0}")]
    ParseError(String),
}

impl From<std::io::Error> for ProjectError {
    fn from(error: std::io::Error) -> Self {
        ProjectError::IoError(error.to_string())
    }
}

impl From<std::num::ParseIntError> for ProjectError {
    fn from(error: std::num::ParseIntError) -> Self {
        ProjectError::ParseError(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for ProjectError {
    fn from(error: std::num::ParseFloatError) -> Self {
        ProjectError::ParseError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ProjectError::DivisionByZero;
        assert_eq!(error.to_string(), "Division durch Null ist nicht erlaubt");
    }

    #[test]
    fn test_overflow_error() {
        let error = ProjectError::OverflowError("Test overflow".to_string());
        assert_eq!(error.to_string(), "Overflow-Fehler: Test overflow");
    }

    #[test]
    fn test_invalid_input_error() {
        let error = ProjectError::InvalidInput("Invalid number".to_string());
        assert_eq!(error.to_string(), "Ungültige Eingabe: Invalid number");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let project_error = ProjectError::from(io_error);
        
        match project_error {
            ProjectError::IoError(msg) => assert!(msg.contains("File not found")),
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_parse_int_error_conversion() {
        let parse_error = "abc".parse::<i32>().unwrap_err();
        let project_error = ProjectError::from(parse_error);
        
        match project_error {
            ProjectError::ParseError(_) => (),
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_parse_float_error_conversion() {
        let parse_error = "xyz".parse::<f64>().unwrap_err();
        let project_error = ProjectError::from(parse_error);
        
        match project_error {
            ProjectError::ParseError(_) => (),
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_error_equality() {
        let error1 = ProjectError::DivisionByZero;
        let error2 = ProjectError::DivisionByZero;
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_clone() {
        let error = ProjectError::DivisionByZero;
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }
}
