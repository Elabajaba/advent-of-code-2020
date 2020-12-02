use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Password {
    pub password: String,
    pub letter: char,
    pub range: (i32, i32),
}

impl Password {
    #[inline(always)]
    pub fn is_in_range(&self, count: i32) -> bool {
        if self.range.0 <= count && count <= self.range.1 {
            return true;
        }
        false
    }
}

fn main() {
    let input = load_file(Path::new("input.txt")).unwrap();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn load_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn parse_passwords(input: &str) -> Vec<Password> {
    let mut passwords = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let letter = split[1].chars().next().unwrap();
        let password = split[2].to_string();
        let ranges: Vec<&str> = split[0].split('-').collect();
        let range: (i32, i32) = (ranges[0].parse().unwrap(), ranges[1].parse().unwrap());
        passwords.push(Password {
            letter,
            password,
            range,
        });
    }

    passwords
}

// Get the number of valid passwords.
fn part1(input: &str) -> i32 {
    let mut valid_passwords = 0;
    let passwords = parse_passwords(input);

    for pword in passwords.iter() {
        let count = pword
            .password
            .chars()
            .fold(0, |acc, c| if c == pword.letter { acc + 1 } else { acc });

        if pword.is_in_range(count) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn part2(input: &str) -> i32 {
    let mut valid_passwords = 0;
    let passwords = parse_passwords(input);

    for pword in passwords.iter() {
        let chars: Vec<char> = pword.password.chars().collect();
        let matches: (bool, bool) = (
            chars[pword.range.0 as usize - 1] == pword.letter,
            chars[pword.range.1 as usize - 1] == pword.letter,
        );

        // ^ is XOR, meaning only true if exactly one is true, otherwise false.
        if matches.0 ^ matches.1 {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_password() {
        let input = "13-15 x: rgnqfdxsvlplxjx\n2-3 g: sxpw".to_string();
        let password = parse_passwords(&input);
        assert_eq!(
            password[0],
            Password {
                password: "rgnqfdxsvlplxjx".to_string(),
                letter: 'x',
                range: (13, 15),
            }
        );
        assert_eq!(
            password[1],
            Password {
                password: "sxpw".to_string(),
                letter: 'g',
                range: (2, 3),
            }
        );
    }
}
