use aoc2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn day1_p1_bench(c: &mut Criterion) {
    let input = get_input(1, false);
    c.bench_function("d1_p1", |b| b.iter(|| day01::Problem.part_one(&input)));
}

fn day1_p2_bench(c: &mut Criterion) {
    let input = get_input(1, false);
    c.bench_function("d1_p2", |b| b.iter(|| day01::Problem.part_two(&input)));
}

fn day2_p1_bench(c: &mut Criterion) {
    let input = get_input(2, false);
    c.bench_function("d2_p1", |b| b.iter(|| day02::Problem.part_one(&input)));
}

fn day2_p2_bench(c: &mut Criterion) {
    let input = get_input(2, false);
    c.bench_function("d2_p2", |b| b.iter(|| day02::Problem.part_two(&input)));
}

fn day3_p1_bench(c: &mut Criterion) {
    let input = get_input(3, false);
    c.bench_function("d3_p1", |b| b.iter(|| day03::Problem.part_one(&input)));
}

fn day3_p2_bench(c: &mut Criterion) {
    let input = get_input(3, false);
    c.bench_function("d3_p2", |b| b.iter(|| day03::Problem.part_two(&input)));
}

fn day4_p1_bench(c: &mut Criterion) {
    let input = get_input(4, false);
    c.bench_function("d4_p1", |b| b.iter(|| day04::Problem.part_one(&input)));
}

fn day4_p2_bench(c: &mut Criterion) {
    let input = get_input(4, false);
    c.bench_function("d4_p2", |b| b.iter(|| day04::Problem.part_two(&input)));
}

fn day5_p1_bench(c: &mut Criterion) {
    let input = get_input(5, false);
    c.bench_function("d5_p1", |b| b.iter(|| day05::Problem.part_one(&input)));
}

fn day5_p2_bench(c: &mut Criterion) {
    let input = get_input(5, false);
    c.bench_function("d5_p2", |b| b.iter(|| day05::Problem.part_one(&input)));
}

pub fn day6_p1_bench(c: &mut Criterion) {
    let input = get_input(6, false);
    c.bench_function("d6_p1", |b| b.iter(|| day06::Problem.part_one(&input)));
}

pub fn day6_p2_bench(c: &mut Criterion) {
    let input = get_input(6, false);
    c.bench_function("d6_p2", |b| b.iter(|| day06::Problem.part_two(&input)));
}

pub fn day7_p1_bench(c: &mut Criterion) {
    let input = get_input(7, false);
    c.bench_function("d7_p1", |b| b.iter(|| day07::Problem.part_one(&input)));
}

pub fn day7_p2_bench(c: &mut Criterion) {
    let input = get_input(7, false);
    c.bench_function("d7_p2", |b| b.iter(|| day07::Problem.part_two(&input)));
}

pub fn day8_p1_bench(c: &mut Criterion) {
    let input = get_input(8, false);
    c.bench_function("d8_p1", |b| b.iter(|| day08::Problem.part_one(&input)));
}

pub fn day8_p2_bench(c: &mut Criterion) {
    let input = get_input(8, false);
    c.bench_function("d8_p2", |b| b.iter(|| day08::Problem.part_two(&input)));
}

pub fn day9_p1_bench(c: &mut Criterion) {
    let input = get_input(9, false);
    c.bench_function("d9_p1", |b| b.iter(|| day09::Problem.part_one(&input)));
}

pub fn day9_p2_bench(c: &mut Criterion) {
    let input = get_input(9, false);
    c.bench_function("d9_p2", |b| b.iter(|| day09::Problem.part_two(&input)));
}

pub fn day10_p1_bench(c: &mut Criterion) {
    let input = get_input(10, false);
    c.bench_function("d10_p1", |b| b.iter(|| day10::Problem.part_one(&input)));
}

pub fn day10_p2_bench(c: &mut Criterion) {
    let input = get_input(10, false);
    c.bench_function("d10_p2", |b| b.iter(|| day10::Problem.part_two(&input)));
}

pub fn day11_p1_bench(c: &mut Criterion) {
    let input = get_input(11, false);
    c.bench_function("d11_p1", |b| b.iter(|| day11::Problem.part_one(&input)));
}

pub fn day11_p2_bench(c: &mut Criterion) {
    let input = get_input(11, false);
    c.bench_function("d11_p2", |b| b.iter(|| day11::Problem.part_two(&input)));
}

criterion_group!(day1_benches, day1_p1_bench, day1_p2_bench);
criterion_group!(day2_benches, day2_p1_bench, day2_p2_bench);
criterion_group!(day3_benches, day3_p1_bench, day3_p2_bench);
criterion_group!(day4_benches, day4_p1_bench, day4_p2_bench);
criterion_group!(day5_benches, day5_p1_bench, day5_p2_bench);
criterion_group!(day6_benches, day6_p1_bench, day6_p2_bench);
criterion_group!(day7_benches, day7_p1_bench, day7_p2_bench);
criterion_group!(day8_benches, day8_p1_bench, day8_p2_bench);
criterion_group!(day9_benches, day9_p1_bench, day9_p2_bench);
criterion_group!(day10_benches, day10_p1_bench, day10_p2_bench);
criterion_group!(day11_benches, day11_p1_bench, day11_p2_bench);
criterion_main!(
    day1_benches,
    day2_benches,
    day3_benches,
    day4_benches,
    day5_benches,
    day6_benches,
    day7_benches,
    day8_benches,
    day9_benches,
    day10_benches,
    day11_benches
);
