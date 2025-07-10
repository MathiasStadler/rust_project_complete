//! Grundlegende Verwendung des Calculators

use rust_project_complete::Calculator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Calculator Beispiel");
    println!("==================");
    
    let mut calc = Calculator::new();
    
    // Grundlegende Operationen
    println!("Grundlegende Operationen:");
    println!("5 + 3 = {}", calc.add(5.0, 3.0));
    println!("10 - 4 = {}", calc.subtract(10.0, 4.0));
    println!("6 * 7 = {}", calc.multiply(6.0, 7.0));
    println!("15 / 3 = {}", calc.divide(15.0, 3.0)?);
    
    // Fakultät
    println!("\nFakultät:");
    println!("5! = {}", calc.factorial(5)?);
    
    // Verlauf anzeigen
    println!("\nVerlauf ({} Operationen):", calc.history_count());
    for (i, op) in calc.get_history().iter().enumerate() {
        println!("{}. {} = {}", i + 1, op.operation_type, op.result);
    }
    
    Ok(())
}
