#![feature(test)]
extern crate test;

use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Obstacles {
    Tree,
    Empty,
}

fn main() {
    let input = load_file(Path::new("input.txt")).unwrap();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn load_file(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_map(input: &str) -> (Vec<Obstacles>, usize) {
    let mut map: Vec<Obstacles> = Vec::new();
    let mut width = 0;
    for line in input.lines() {
        width = line.len();
        for c in line.chars() {
            map.push(match c {
                '#' => Obstacles::Tree,
                '.' => Obstacles::Empty,
                _ => panic!("Expected either '#' or '.', received {:?}", c),
            });
        }
    }
    (map, width)
}

#[inline(always)]
fn get_trees_hit(map: &[Obstacles], run: usize, rise: usize, width: usize) -> i64 {
    // let mut trees_hit = 0;
    // for (i, line) in map.chunks(width).step_by(rise).enumerate() {
    //     let x = (i * run) % width;
    //     if line[x] == Obstacles::Tree {
    //         trees_hit += 1;
    //     }
    // }
    // trees_hit
    map.chunks(width)
        .step_by(rise)
        .enumerate()
        .fold(0_i64, |acc, (i, line)| {
            if line[(i * run) % width] == Obstacles::Tree {
                acc + 1
            } else {
                acc
            }
        })
}

#[inline(always)]
fn get_trees_hit_multithreaded(map: &[Obstacles], run: usize, rise: usize, width: usize) -> i64 {
    map.par_chunks(width)
        .step_by(rise)
        .enumerate()
        .fold(
            || 0_i64,
            |acc, (i, line)| {
                if line[(i * run) % width] == Obstacles::Tree {
                    acc + 1
                } else {
                    acc
                }
            },
        )
        .sum::<i64>()
}

fn part1(input: &str) -> i64 {
    let (map, width) = parse_map(&input);

    get_trees_hit_multithreaded(&map, 3, 1, width)
}

fn part2(input: &str) -> i64 {
    let (map, width) = parse_map(&input);
    get_trees_hit_multithreaded(&map, 1, 1, width)
        * get_trees_hit_multithreaded(&map, 3, 1, width)
        * get_trees_hit_multithreaded(&map, 5, 1, width)
        * get_trees_hit_multithreaded(&map, 7, 1, width)
        * get_trees_hit_multithreaded(&map, 1, 2, width)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_parse_map() {
        let input = "..#\n##.".to_string();
        let (map, _width) = parse_map(&input);
        assert_eq!(
            map,
            vec![
                Obstacles::Empty,
                Obstacles::Empty,
                Obstacles::Tree,
                Obstacles::Tree,
                Obstacles::Tree,
                Obstacles::Empty
            ]
        );
    }

    #[test]
    fn test_get_trees_hit_multithreaded() {
        // \n\ at end of line for nicer indentation
        let input = "..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#";
        let (map, width) = parse_map(&input);

        let trees_hit = get_trees_hit_multithreaded(&map, 1, 1, width);
        assert_eq!(trees_hit, 2);
        let trees_hit = get_trees_hit_multithreaded(&map, 3, 1, width);
        assert_eq!(trees_hit, 7);
        let trees_hit = get_trees_hit_multithreaded(&map, 5, 1, width);
        assert_eq!(trees_hit, 3);
        let trees_hit = get_trees_hit_multithreaded(&map, 7, 1, width);
        assert_eq!(trees_hit, 4);
        let trees_hit = get_trees_hit_multithreaded(&map, 1, 2, width);
        assert_eq!(trees_hit, 2);
    }

    #[test]
    fn test_get_part_2() {
        // \n\ at end of line for nicer indentation
        let input = "..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#";

        let part2_product = part2(input);
        assert_eq!(part2_product, 336);
    }

    #[bench]
    fn bench_parse_map(b: &mut Bencher) {
        // \n\ at end of line for nicer indentation
        let input = load_file(Path::new("input.txt")).unwrap();
        b.iter(|| parse_map(&input))
    }

    #[bench]
    fn bench_get_trees_hit(b: &mut Bencher) {
        // \n\ at end of line for nicer indentation
        let input = load_file(Path::new("input.txt")).unwrap();
        let (map, width) = parse_map(&input);
        b.iter(|| get_trees_hit(&map, 3, 1, width))
    }

    #[bench]
    fn bench_get_trees_hit_multithreaded(b: &mut Bencher) {
        // \n\ at end of line for nicer indentation
        let input = load_file(Path::new("input.txt")).unwrap();
        let (map, width) = parse_map(&input);
        b.iter(|| get_trees_hit_multithreaded(&map, 3, 1, width))
    }
}
