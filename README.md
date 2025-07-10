# Rust Project Complete

Ein vollständiges Rust-Projekt mit 100% Testabdeckung, das als Best Practice Beispiel für professionelle Rust-Entwicklung dient.

## Features

- 🧮 **Calculator**: Vollständiger Rechner mit Grundoperationen
- 📊 **Statistiken**: Detaillierte Operationsstatistiken
- 🔧 **Utilities**: Nützliche Hilfsfunktionen
- ✅ **100% Testabdeckung**: Unit-, Integration- und Property-based Tests
- 🚀 **Performance**: Benchmark-Tests für kritische Pfade
- 📚 **Dokumentation**: Vollständige API-Dokumentation
- 🔒 **Sicherheit**: Regelmäßige Dependency-Audits

## Installation

```bash
git clone https://github.com/yourusername/rust_project_complete.git
cd rust_project_complete
cargo build --release
```

## Verwendung

### Als Bibliothek

```rust
use rust_project_complete::Calculator;

let mut calc = Calculator::new();
let result = calc.add(2.0, 3.0);
println!("2 + 3 = {}", result);
```

### Als Kommandozeilen-Tool

```bash
# Grundlegende Operationen
cargo run -- add 5 3
cargo run -- multiply 4 7
cargo run -- factorial 5

# Interaktiver Modus
cargo run -- interactive
```

### Beispiele ausführen

```bash
cargo run --example basic_usage
cargo run --example advanced_usage
```

## Entwicklung

### Tests ausführen

```bash
# Alle Tests
cargo test

# Mit Coverage
cargo tarpaulin --out Html --output-dir coverage/

# Benchmarks
cargo bench
```

### Code-Qualität

```bash
# Linting
cargo clippy -- -D warnings

# Formatierung
cargo fmt

# Sicherheitsaudit
cargo audit
```

## Projektstruktur

```
src/
├── lib.rs              # Hauptbibliothek
├── main.rs             # CLI-Anwendung
├── modules/
│   ├── core.rs         # Kernfunktionalität
│   ├── error.rs        # Fehlertypen
│   └── utils.rs        # Hilfsfunktionen
└── bin/
    └── example.rs      # Beispiel-Binary

tests/
├── integration_tests.rs # Integrationstests
└── common/             # Test-Utilities

benches/
└── benchmarks.rs       # Performance-Tests

examples/
├── basic_usage.rs      # Grundlegende Verwendung
└── advanced_usage.rs   # Erweiterte Funktionen
```

## API-Dokumentation

```bash
cargo doc --open
```

## Lizenz

Dieses Projekt steht unter der MIT oder Apache-2.0 Lizenz.

## Beitragen

1. Fork des Repositories
2. Feature-Branch erstellen
3. Änderungen committen
4. Tests ausführen
5. Pull Request erstellen

## Qualitätsmetriken

- **Testabdeckung**: 100%
- **Dokumentation**: Vollständig
- **Performance**: Benchmarked
- **Sicherheit**: Regelmäßig auditiert
