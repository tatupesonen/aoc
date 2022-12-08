use aoc2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn day1_p1_bench(c: &mut Criterion) {
    let input = get_input(1, false);
    c.bench_function("d1_p1", |b| b.iter(|| day1::Problem.part_one(&input)));
}

fn day1_p2_bench(c: &mut Criterion) {
    let input = get_input(1, false);
    c.bench_function("d1_p2", |b| b.iter(|| day1::Problem.part_two(&input)));
}

fn day2_p1_bench(c: &mut Criterion) {
    let input = get_input(2, false);
    c.bench_function("d2_p1", |b| b.iter(|| day2::Problem.part_one(&input)));
}

fn day2_p2_bench(c: &mut Criterion) {
    let input = get_input(2, false);
    c.bench_function("d2_p2", |b| b.iter(|| day2::Problem.part_two(&input)));
}

fn day3_p1_bench(c: &mut Criterion) {
    let input = get_input(3, false);
    c.bench_function("d3_p1", |b| b.iter(|| day3::Problem.part_one(&input)));
}

fn day3_p2_bench(c: &mut Criterion) {
    let input = get_input(3, false);
    c.bench_function("d3_p2", |b| b.iter(|| day3::Problem.part_two(&input)));
}

fn day4_p1_bench(c: &mut Criterion) {
    let input = get_input(4, false);
    c.bench_function("d4_p1", |b| b.iter(|| day4::Problem.part_one(&input)));
}

fn day4_p2_bench(c: &mut Criterion) {
    let input = get_input(4, false);
    c.bench_function("d4_p2", |b| b.iter(|| day4::Problem.part_two(&input)));
}

fn day5_p1_bench(c: &mut Criterion) {
    let input = get_input(5, false);
    c.bench_function("d5_p1", |b| b.iter(|| day5::Problem.part_one(&input)));
}

fn day5_p2_bench(c: &mut Criterion) {
    let input = get_input(5, false);
    c.bench_function("d5_p2", |b| b.iter(|| day5::Problem.part_one(&input)));
}

pub fn day6_p1_bench(c: &mut Criterion) {
    let input = get_input(6, false);
    c.bench_function("d6_p1", |b| b.iter(|| day6::Problem.part_one(&input)));
}

pub fn day6_p2_bench(c: &mut Criterion) {
    let input = get_input(6, false);
    c.bench_function("d6_p2", |b| b.iter(|| day6::Problem.part_two(&input)));
}

pub fn day7_p1_bench(c: &mut Criterion) {
    let input = get_input(7, false);
    c.bench_function("d7_p1", |b| b.iter(|| day7::Problem.part_one(&input)));
}

pub fn day7_p2_bench(c: &mut Criterion) {
    let input = get_input(7, false);
    c.bench_function("d7_p2", |b| b.iter(|| day7::Problem.part_two(&input)));
}

pub fn day8_p1_bench(c: &mut Criterion) {
    let input = get_input(8, false);
    c.bench_function("d8_p1", |b| b.iter(|| day8::Problem.part_one(&input)));
}

pub fn day8_p2_bench(c: &mut Criterion) {
    let input = get_input(8, false);
    c.bench_function("d8_p2", |b| b.iter(|| day8::Problem.part_two(&input)));
}

criterion_group!(day1_benches, day1_p1_bench, day1_p2_bench);
criterion_group!(day2_benches, day2_p1_bench, day2_p2_bench);
criterion_group!(day3_benches, day3_p1_bench, day3_p2_bench);
criterion_group!(day4_benches, day4_p1_bench, day4_p2_bench);
criterion_group!(day5_benches, day5_p1_bench, day5_p2_bench);
criterion_group!(day6_benches, day6_p1_bench, day6_p2_bench);
criterion_group!(day7_benches, day7_p1_bench, day7_p2_bench);
criterion_group!(day8_benches, day8_p1_bench, day8_p2_bench);
criterion_main!(
    day1_benches,
    day2_benches,
    day3_benches,
    day4_benches,
    day5_benches,
    day6_benches,
    day7_benches,
    day8_benches
);
