//! Benchmark-Tests

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_project_complete::Calculator;
use rust_project_complete::modules::core::MathUtils;
use rust_project_complete::modules::utils::*;

fn benchmark_calculator_operations(c: &mut Criterion) {
    let mut calc = Calculator::new();
    
    c.bench_function("add", |b| {
        b.iter(|| calc.add(black_box(1.0), black_box(2.0)))
    });
    
    c.bench_function("multiply", |b| {
        b.iter(|| calc.multiply(black_box(3.0), black_box(4.0)))
    });
    
    c.bench_function("divide", |b| {
        b.iter(|| calc.divide(black_box(10.0), black_box(2.0)))
    });
    
    c.bench_function("factorial", |b| {
        b.iter(|| calc.factorial(black_box(10)))
    });
}

fn benchmark_math_utils(c: &mut Criterion) {
    c.bench_function("gcd", |b| {
        b.iter(|| MathUtils::gcd(black_box(48), black_box(18)))
    });
    
    c.bench_function("lcm", |b| {
        b.iter(|| MathUtils::lcm(black_box(48), black_box(18)))
    });
    
    c.bench_function("is_prime", |b| {
        b.iter(|| MathUtils::is_prime(black_box(97)))
    });
    
    c.bench_function("fibonacci", |b| {
        b.iter(|| MathUtils::fibonacci(black_box(20)))
    });
}

fn benchmark_utils(c: &mut Criterion) {
    let numbers: Vec<f64> = (1..=1000).map(|i| i as f64).collect();
    
    c.bench_function("calculate_average", |b| {
        b.iter(|| calculate_average(black_box(&numbers)))
    });
    
    c.bench_function("find_minimum", |b| {
        b.iter(|| find_minimum(black_box(&numbers)))
    });
    
    c.bench_function("find_maximum", |b| {
        b.iter(|| find_maximum(black_box(&numbers)))
    });
    
    c.bench_function("calculate_standard_deviation", |b| {
        b.iter(|| calculate_standard_deviation(black_box(&numbers)))
    });
}

criterion_group!(
    benches,
    benchmark_calculator_operations,
    benchmark_math_utils,
    benchmark_utils
);
criterion_main!(benches);
