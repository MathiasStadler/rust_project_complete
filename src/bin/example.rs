//! Beispiel-Binary

use rust_project_complete::Calculator;

fn main() {
    let mut calc = Calculator::new();
    
    println!("Rust Calculator Beispiel");
    println!("========================");
    
    // Einige Berechnungen
    let result1 = calc.add(10.0, 5.0);
    println!("10 + 5 = {}", result1);
    
    let result2 = calc.multiply(3.0, 4.0);
    println!("3 * 4 = {}", result2);
    
    match calc.divide(20.0, 4.0) {
        Ok(result) => println!("20 / 4 = {}", result),
        Err(e) => eprintln!("Fehler: {}", e),
    }
    
    match calc.factorial(5) {
        Ok(result) => println!("5! = {}", result),
        Err(e) => eprintln!("Fehler: {}", e),
    }
    
    println!("\nVerlauf: {} Operationen", calc.history_count());
    
    for (i, op) in calc.get_history().iter().enumerate() {
        println!("{}. {} = {}", i + 1, op.operation_type, op.result);
    }
}
