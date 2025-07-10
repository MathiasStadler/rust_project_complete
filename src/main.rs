//! Hauptprogramm

use rust_project_complete::{Calculator, OperationType};
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "rust_project_complete")]
#[command(about = "Ein vollständiger Rust-Calculator mit Tests")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Addiert zwei Zahlen
    Add {
        /// Erste Zahl
        a: f64,
        /// Zweite Zahl
        b: f64,
    },
    /// Subtrahiert zwei Zahlen
    Subtract {
        /// Erste Zahl
        a: f64,
        /// Zweite Zahl
        b: f64,
    },
    /// Multipliziert zwei Zahlen
    Multiply {
        /// Erste Zahl
        a: f64,
        /// Zweite Zahl
        b: f64,
    },
    /// Dividiert zwei Zahlen
    Divide {
        /// Dividend
        a: f64,
        /// Divisor
        b: f64,
    },
    /// Berechnet die Fakultät
    Factorial {
        /// Zahl für Fakultät
        n: u64,
    },
    /// Interaktiver Modus
    Interactive,
}

fn main() {
    env_logger::init();
    
    let cli = Cli::parse();
    let mut calc = Calculator::new();
    
    match cli.command {
        Commands::Add { a, b } => {
            let result = calc.add(a, b);
            println!("{} + {} = {}", a, b, result);
        }
        Commands::Subtract { a, b } => {
            let result = calc.subtract(a, b);
            println!("{} - {} = {}", a, b, result);
        }
        Commands::Multiply { a, b } => {
            let result = calc.multiply(a, b);
            println!("{} * {} = {}", a, b, result);
        }
        Commands::Divide { a, b } => {
            match calc.divide(a, b) {
                Ok(result) => println!("{} / {} = {}", a, b, result),
                Err(e) => {
                    eprintln!("Fehler: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Factorial { n } => {
            match calc.factorial(n) {
                Ok(result) => println!("{}! = {}", n, result),
                Err(e) => {
                    eprintln!("Fehler: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Interactive => {
            println!("Interaktiver Modus - Implementierung folgt...");
        }
    }
}
