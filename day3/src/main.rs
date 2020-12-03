#![feature(test)]
extern crate test;

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

fn parse_map(input: &str) -> Vec<Vec<Obstacles>> {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut inner: Vec<Obstacles> = Vec::with_capacity(line.len());
        for c in line.chars() {
            inner.push(match c {
                '#' => Obstacles::Tree,
                '.' => Obstacles::Empty,
                _ => panic!("unexpected input. expects # or ., but received: {}", c),
            });
        }
        map.push(inner);
    }
    map
}

#[inline(always)]
fn get_trees_hit(map: &[Vec<Obstacles>], run: usize, rise: usize) -> i64 {
    let mut trees_hit = 0;
    let width = map[0].len();
    let mut x = 0;
    for line in map.iter().step_by(rise) {
        if line[x] == Obstacles::Tree {
            trees_hit += 1;
        }
        x += run;
        if x >= width {
            x -= width;
        }
    }
    trees_hit
}

fn part1(input: &str) -> i64 {
    let map = parse_map(&input);

    get_trees_hit(&map, 3, 1)
}

fn part2(input: &str) -> i64 {
    let map = parse_map(&input);
    get_trees_hit(&map, 1, 1)
        * get_trees_hit(&map, 3, 1)
        * get_trees_hit(&map, 5, 1)
        * get_trees_hit(&map, 7, 1)
        * get_trees_hit(&map, 1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_parse_map() {
        let input = "..#\n##.".to_string();
        let map = parse_map(&input);
        assert_eq!(
            map[0],
            vec![Obstacles::Empty, Obstacles::Empty, Obstacles::Tree]
        );
        assert_eq!(
            map[1],
            vec![Obstacles::Tree, Obstacles::Tree, Obstacles::Empty]
        );
    }

    #[test]
    fn test_get_trees_hit() {
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
        let map = parse_map(input);

        let trees_hit = get_trees_hit(&map, 1, 1);
        assert_eq!(trees_hit, 2);
        let trees_hit = get_trees_hit(&map, 3, 1);
        assert_eq!(trees_hit, 7);
        let trees_hit = get_trees_hit(&map, 5, 1);
        assert_eq!(trees_hit, 3);
        let trees_hit = get_trees_hit(&map, 7, 1);
        assert_eq!(trees_hit, 4);
        let trees_hit = get_trees_hit(&map, 1, 2);
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
        let map = parse_map(&input);
        b.iter(|| get_trees_hit(&map, 3, 1))
    }
}
