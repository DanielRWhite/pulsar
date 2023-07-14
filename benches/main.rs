use criterion::{black_box, criterion_group, criterion_main, Criterion};
use half::{ f16, bf16 };

fn f16_halve_operation(initial_value: f16, num_halves: i32) -> f16 {
        let x = 1.0_f64.powi(num_halves);

        initial_value * (f16::from_f32(1.0_f32) / f16::from_f64(x))
}

fn bf16_halve_operation(initial_value: bf16, num_halves: i32) -> bf16 {
        let x = 1.0_f64.powi(num_halves);

        initial_value * (bf16::from_f32(1.0_f32) / bf16::from_f64(x))
}

fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("[F16] Halving Equation (1 halving)", |b| b.iter(|| f16_halve_operation(black_box(f16::from_f32(1.0_f32)), black_box(1))));
        c.bench_function("[BF16] Halving Equation (1 halving)", |b| b.iter(|| bf16_halve_operation(black_box(bf16::from_f32(1.0_f32)), black_box(1))));

        c.bench_function("[F16] Halving Equation (2 halvings)", |b| b.iter(|| f16_halve_operation(black_box(f16::from_f32(1.0_f32)), black_box(2))));
        c.bench_function("[BF16] Halving Equation (2 halvings)", |b| b.iter(|| bf16_halve_operation(black_box(bf16::from_f32(1.0_f32)), black_box(2))));

        c.bench_function("[F16] Halving Equation (4 halvings)", |b| b.iter(|| f16_halve_operation(black_box(f16::from_f32(1.0_f32)), black_box(4))));
        c.bench_function("[BF16] Halving Equation (4 halvings)", |b| b.iter(|| bf16_halve_operation(black_box(bf16::from_f32(1.0_f32)), black_box(4))));

        c.bench_function("[F16] Halving Equation (8 halvings)", |b| b.iter(|| f16_halve_operation(black_box(f16::from_f32(1.0_f32)), black_box(8))));
        c.bench_function("[BF16] Halving Equation (8 halvings)", |b| b.iter(|| bf16_halve_operation(black_box(bf16::from_f32(1.0_f32)), black_box(8))));

        c.bench_function("[F16] Halving Equation (16 halvings)", |b| b.iter(|| f16_halve_operation(black_box(f16::from_f32(1.0_f32)), black_box(16))));
        c.bench_function("[BF16] Halving Equation (16 halvings)", |b| b.iter(|| bf16_halve_operation(black_box(bf16::from_f32(1.0_f32)), black_box(16))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);