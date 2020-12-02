#![feature(test)]
#![feature(drain_filter)]
extern crate test;

use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

fn main() {
    let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
    // find_two_numbers_sort_filter(integers);
    // part1(&integers);
    // part2(&integers);
}

fn load_file_to_vec(path: &Path) -> Result<Vec<i32>> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let mut integers = Vec::new();
    for line in input.lines() {
        integers.push(line.parse::<i32>().unwrap());
    }
    Ok(integers)
}

fn part1(input: &[i32]) {
    let numbers = find_two_numbers_original_optimized(input).unwrap();
    println!("{}, {}", numbers.0, numbers.1);
}

#[inline(always)]
fn find_two_numbers(input: &[i32]) -> Option<(i32, i32)> {
    for (i, int1) in input.iter().enumerate() {
        for (j, int2) in input.iter().enumerate() {
            if i == j {
                continue;
            }
            if int1 + int2 == 2020 {
                return Some((*int1, *int2));
            }
        }
    }

    None
}

#[inline(always)]
fn find_two_numbers_original_optimized(input: &[i32]) -> Option<(i32, i32)> {
    for (i, int1) in input.iter().enumerate() {
        for int2 in input[i..].iter() {
            if int1 + int2 == 2020 {
                return Some((*int1, *int2));
            }
        }
    }

    None
}

#[inline(always)]
fn find_two_numbers_basic(input: &[i32]) -> (i32, i32) {
    let len = input.len();
    for i in 0..len {
        for j in i..len {
            if input[i] + input[j] == 2020 {
                return (input[i], input[j]);
            }
        }
    }
    (0, 0)
}

#[inline(always)]
fn find_two_numbers_sort_filter(mut input: Vec<i32>) -> Option<(i32, i32)> {
    // Sort the input
    input.sort_by(|a, b| b.cmp(a));
    // Filter out the numbers that are too big (eg. smallest + x > 2020)
    let smallest = input[input.len() - 1];
    input.drain_filter(|a| *a + smallest > 2020);

    for int1 in input.iter() {
        for int2 in input.iter().rev() {
            if (int1 + int2) > 2020 {
                // It's too big, skip to the next bunch.
                break;
            }
            if (int1 + int2) == 2020 {
                return Some((*int1, *int2));
            }
        }
    }

    None
}

fn part2(input: &[i32]) {
    let numbers = find_three_numbers_original_optimized(input).unwrap();
    println!("{}, {}, {}", numbers.0, numbers.1, numbers.2);
}

#[inline(always)]
fn find_three_numbers(input: &[i32]) -> Option<(i32, i32, i32)> {
    for (i, int1) in input.iter().enumerate() {
        for (j, int2) in input.iter().enumerate() {
            for (k, int3) in input.iter().enumerate() {
                if i == j || i == k {
                    continue;
                }
                if int1 + int2 + int3 == 2020 {
                    return Some((*int1, *int2, *int3));
                }
            }
        }
    }

    None
}

#[inline(always)]
fn find_three_numbers_original_optimized(input: &[i32]) -> Option<(i32, i32, i32)> {
    for (i, int1) in input.iter().enumerate() {
        for (j, int2) in input[i..].iter().enumerate() {
            for int3 in input[i + j..].iter() {
                if int1 + int2 + int3 == 2020 {
                    return Some((*int1, *int2, *int3));
                }
            }
        }
    }

    None
}

#[inline(always)]
fn find_three_numbers_sort_filter(mut input: Vec<i32>) -> Option<(i32, i32, i32)> {
    // Sort the input
    input.sort_by(|a, b| b.cmp(a));
    // Filter out the numbers that are too big (eg. smallest + x > 2020)
    let smallest = input[input.len() - 1];
    let second_smallest = input[input.len() - 2];
    input.drain_filter(|a| *a + smallest + second_smallest > 2020);

    for (i, int1) in input.iter().enumerate() {
        for (j, int2) in input[i..].iter().enumerate() {
            if (int1 + int2) >= 2020 {
                // It's too big, skip to the next bunch.
                break;
            }
            for int3 in input[i + j..].iter().rev() {
                if (int1 + int2 + int3) > 2020 {
                    // It's too big, skip to the next bunch.
                    break;
                }

                if (int1 + int2 + int3) == 2020 {
                    return Some((*int1, *int2, *int3));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn find_two_all_work() {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        let original = find_two_numbers(&integers);
        let optimized = find_two_numbers_original_optimized(&integers);
        let basic = find_two_numbers_basic(&integers);
        let filter_fancy = find_two_numbers_sort_filter(integers);
        assert_eq!(original, optimized);
        assert_eq!(Some(basic), optimized);
        assert_eq!(filter_fancy, original);
    }

    #[test]
    fn find_three_all_work() {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        let original = find_three_numbers(&integers);
        let optimized = find_three_numbers_original_optimized(&integers);
        let filter_fancy = find_three_numbers_sort_filter(integers);
        assert_eq!(original, optimized);
        assert_eq!(filter_fancy, original);
    }

    #[bench]
    fn bench_find_two_numbers(b: &mut Bencher) {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        b.iter(|| find_two_numbers(&integers))
    }

    #[bench]
    fn bench_find_two_numbers_optimized(b: &mut Bencher) {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        b.iter(|| find_two_numbers_original_optimized(&integers))
    }

    #[bench]
    fn bench_find_two_numbers_basic(b: &mut Bencher) {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        b.iter(|| find_two_numbers_basic(&integers))
    }

    #[bench]
    fn bench_find_two_numbers_sort_filter(b: &mut Bencher) {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        b.iter(|| find_two_numbers_sort_filter(integers.clone()))
    }

    #[bench]
    fn bench_find_three_numbers(b: &mut Bencher) {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        b.iter(|| find_three_numbers(&integers))
    }

    #[bench]
    fn bench_find_three_numbers_optimized(b: &mut Bencher) {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        b.iter(|| find_three_numbers_original_optimized(&integers))
    }

    #[bench]
    fn bench_find_three_numbers_sort_filter(b: &mut Bencher) {
        let integers = load_file_to_vec(Path::new("input.txt")).unwrap();
        b.iter(|| find_three_numbers_sort_filter(integers.clone()))
    }
}
