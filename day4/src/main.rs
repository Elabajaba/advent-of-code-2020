use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

// byr // (Birth Year)
// iyr // (Issue Year)
// eyr // (Expiration Year)
// hgt // (Height)
// hcl // (Hair Color)
// ecl // (Eye Color)
// pid // (Passport ID)
// cid // (Country ID)

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Passport {
    pub byr: Option<String>, // (Birth Year)
    pub iyr: Option<String>, // (Issue Year)
    pub eyr: Option<String>, // (Expiration Year)
    pub hgt: Option<String>, // (Height)
    pub hcl: Option<String>, // (Hair Color)
    pub ecl: Option<String>, // (Eye Color)
    pub pid: Option<String>, // (Passport ID)
    pub cid: Option<String>, // (Country ID)
}

impl Passport {
    pub fn empty() -> Passport {
        Passport {
            byr: None, // (Birth Year)
            iyr: None, // (Issue Year)
            eyr: None, // (Expiration Year)
            hgt: None, // (Height)
            hcl: None, // (Hair Color)
            ecl: None, // (Eye Color)
            pid: None, // (Passport ID)
            cid: None, // (Country ID)
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        {
            return true;
        }

        false
    }

    pub fn validate(&self) -> bool {
        self.validate_birth_year()
            && self.validate_issue_year()
            && self.validate_expiration_year()
            && self.validate_height()
            && self.validate_hair_color()
            && self.validate_eye_color()
            && self.validate_passport_id()
    }

    fn validate_value_in_range(range: (&u16, &u16), value: &u16) -> bool {
        value >= range.0 && value <= range.1
    }

    fn validate_birth_year(&self) -> bool {
        match &self.byr {
            Some(byr) => {
                // For inputs with leading zeroes (eg. 02001) which get dropped by parsing to a number.
                if byr.len() != 4 {
                    return false;
                }
                let birth_year = match byr.parse::<u16>() {
                    Err(_e) => return false,
                    Ok(val) => val,
                };
                Passport::validate_value_in_range((&1920, &2002), &birth_year)
            }
            _ => false,
        }
    }

    fn validate_issue_year(&self) -> bool {
        match &self.iyr {
            Some(iyr) => {
                // For inputs with leading zeroes (eg. 02013) which get dropped by parsing to a number.
                if iyr.len() != 4 {
                    return false;
                }
                let issue_year = match iyr.parse::<u16>() {
                    Err(_e) => return false,
                    Ok(val) => val,
                };
                Passport::validate_value_in_range((&2010, &2020), &issue_year)
            }
            _ => false,
        }
    }

    fn validate_expiration_year(&self) -> bool {
        match &self.eyr {
            Some(eyr) => {
                // For inputs with leading zeroes (eg. 02023) which get dropped by parsing to a number.
                if eyr.len() != 4 {
                    return false;
                }
                let expiration_year = match eyr.parse::<u16>() {
                    Err(_e) => return false,
                    Ok(val) => val,
                };
                Passport::validate_value_in_range((&2020, &2030), &expiration_year)
            }
            _ => false,
        }
    }

    fn validate_height(&self) -> bool {
        match &self.hgt {
            Some(hgt) => {
                if hgt.ends_with("cm") {
                    let height_str = match hgt.strip_suffix("cm") {
                        None => return false,
                        Some(s) => s,
                    };
                    // For inputs with leading zeroes (eg. 0151) which get dropped by parsing to a number.
                    if height_str.len() != 3 {
                        return false;
                    }
                    let height_num = match height_str.parse::<u16>() {
                        Err(_e) => return false,
                        Ok(val) => val,
                    };
                    Passport::validate_value_in_range((&150, &193), &height_num)
                } else if hgt.ends_with("in") {
                    let height_str = match hgt.strip_suffix("in") {
                        None => return false,
                        Some(s) => s,
                    };
                    // For inputs with leading zeroes (eg. 0059) which get dropped by parsing to a number.
                    if height_str.len() != 2 {
                        return false;
                    }
                    let height_num = match height_str.parse::<u16>() {
                        Err(_e) => return false,
                        Ok(val) => val,
                    };
                    Passport::validate_value_in_range((&59, &76), &height_num)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn validate_hair_color(&self) -> bool {
        let valid_chars = vec![
            'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        ];
        match &self.hcl {
            Some(hcl) => {
                let mut chars = hcl.chars();
                if hcl.chars().count() != 7 {
                    // Wrong length
                    return false;
                }
                if chars.next().unwrap() != '#' {
                    // Wrong first character
                    return false;
                }
                for c in chars {
                    if !valid_chars.contains(&c) {
                        // Invalid character
                        return false;
                    }
                }

                true
            }
            _ => false,
        }
    }

    fn validate_eye_color(&self) -> bool {
        let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match &self.ecl {
            Some(ecl) => valid_eye_colors.contains(&ecl.as_str()),
            _ => false,
        }
    }

    fn validate_passport_id(&self) -> bool {
        match &self.pid {
            Some(pid) => {
                let mut is_valid = true;
                if pid.chars().count() != 9 {
                    return false;
                }
                for c in pid.chars() {
                    if !c.is_digit(10) {
                        is_valid = false;
                    }
                }
                is_valid
            }
            _ => false,
        }
    }
}

// Parsing input:
// Split on newlines, then split on whitespace, then split on semicolons.
// Empty newline is the end of that passport.
fn parse_passports(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut current_passport = Passport::empty();
    let length = input.lines().count();
    for (i, line) in input.lines().enumerate() {
        let elements: Vec<&str> = line.split_whitespace().collect();
        for element in elements {
            let fields: Vec<&str> = element.split(':').collect();
            match fields[0] {
                field if field == "byr" => current_passport.byr = Some(fields[1].to_string()),
                field if field == "iyr" => current_passport.iyr = Some(fields[1].to_string()),
                field if field == "eyr" => current_passport.eyr = Some(fields[1].to_string()),
                field if field == "hgt" => current_passport.hgt = Some(fields[1].to_string()),
                field if field == "hcl" => current_passport.hcl = Some(fields[1].to_string()),
                field if field == "ecl" => current_passport.ecl = Some(fields[1].to_string()),
                field if field == "pid" => current_passport.pid = Some(fields[1].to_string()),
                field if field == "cid" => current_passport.cid = Some(fields[1].to_string()),
                _ => panic!("Invalid input"),
            }
        }

        // End of a passport. Add current_passport to the list and reset it.
        if line.is_empty() || i == length - 1 {
            passports.push(current_passport);
            current_passport = Passport::empty();
        }
    }
    passports
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

// Count valid passports using simple passport.is_valid() validation
// is_valid() only checks if the field exists
fn part1(input: &str) -> i32 {
    parse_passports(input)
        .iter()
        .fold(0, |acc, passport| match passport.is_valid() {
            true => acc + 1,
            false => acc,
        })
}

// Count valid passports using the more complex passport.validate()
// passport.validate() validates all fields but cid match their criteria
fn part2(input: &str) -> i32 {
    parse_passports(input)
        .iter()
        .fold(0, |acc, passport| match passport.validate() {
            true => acc + 1,
            false => acc,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_passports() {
        let input = "hcl:#b5c3db ecl:grn hgt:155cm pid:#baec97 iyr:2017\n\
            byr:1939\n\
            eyr:2020\n\
            \n\
            ecl:oth\n\
            \n\
            cid:277";
        let parsed_input = parse_passports(input);
        assert_eq!(
            parsed_input[0],
            Passport {
                byr: Some("1939".to_string()),    // (Birth Year)
                iyr: Some("2017".to_string()),    // (Issue Year)
                eyr: Some("2020".to_string()),    // (Expiration Year)
                hgt: Some("155cm".to_string()),   // (Height)
                hcl: Some("#b5c3db".to_string()), // (Hair Color)
                ecl: Some("grn".to_string()),     // (Eye Color)
                pid: Some("#baec97".to_string()), // (Passport ID)
                cid: None,                        // (Country ID)
            }
        );
        assert_eq!(
            parsed_input[1],
            Passport {
                byr: None,                    // (Birth Year)
                iyr: None,                    // (Issue Year)
                eyr: None,                    // (Expiration Year)
                hgt: None,                    // (Height)
                hcl: None,                    // (Hair Color)
                ecl: Some("oth".to_string()), // (Eye Color)
                pid: None,                    // (Passport ID)
                cid: None,                    // (Country ID)
            }
        );
        assert_eq!(
            parsed_input[2],
            Passport {
                byr: None,                    // (Birth Year)
                iyr: None,                    // (Issue Year)
                eyr: None,                    // (Expiration Year)
                hgt: None,                    // (Height)
                hcl: None,                    // (Hair Color)
                ecl: None,                    // (Eye Color)
                pid: None,                    // (Passport ID)
                cid: Some("277".to_string()), // ()                        // (Country ID)
            }
        );
    }

    #[test]
    fn test_part_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
            byr:1937 iyr:2017 cid:147 hgt:183cm\n\
            \n\
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
            hcl:#cfa07d byr:1929\n\
            \n\
            hcl:#ae17e1 iyr:2013\n\
            eyr:2024\n\
            ecl:brn pid:760753108 byr:1931\n\
            hgt:179cm\n\
            \n\
            hcl:#cfa07d eyr:2025 pid:166559648\n\
            iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_passport_validate_birth_year() {
        let mut passport = Passport::empty();
        let valid_birth_years = vec!["2002".to_string(), "1920".to_string()];
        let invalid_birth_years = vec![
            "2003".to_string(),
            "100000".to_string(),
            "j200".to_string(),
            "02001".to_string(),
            "2000.0".to_string()
        ];

        for year in valid_birth_years {
            passport.byr = Some(year);
            assert_eq!(
                passport.validate_birth_year(),
                true,
                "birth year: {:?}",
                &passport.byr
            );
        }
        for invalid_year in invalid_birth_years {
            passport.byr = Some(invalid_year);
            assert_eq!(
                passport.validate_birth_year(),
                false,
                "birth year: {:?}",
                &passport.byr
            );
        }
    }
    #[test]
    fn test_passport_validate_issue_year() {
        let mut passport = Passport::empty();
        let valid_issue_years = vec!["2010".to_string(), "2020".to_string()];
        let invalid_issue_years =
            vec!["2021".to_string(), "100000".to_string(), "j200".to_string(), "02030".to_string(), "02021.1".to_string()];

        for year in valid_issue_years {
            passport.iyr = Some(year);
            assert_eq!(
                passport.validate_issue_year(),
                true,
                "issue year: {:?}",
                &passport.iyr
            );
        }
        for invalid_year in invalid_issue_years {
            passport.iyr = Some(invalid_year);
            assert_eq!(
                passport.validate_issue_year(),
                false,
                "issue year: {:?}",
                &passport.iyr
            );
        }
    }

    #[test]
    fn test_passport_validate_expiration_year() {
        let mut passport = Passport::empty();
        let valid_expiration_years = vec!["2020".to_string(), "2030".to_string()];
        let invalid_expiration_years =
            vec!["2031".to_string(), "100000".to_string(), "j200".to_string(), "02021".to_string(), "2021.0".to_string()];

        for year in valid_expiration_years {
            passport.eyr = Some(year);
            assert_eq!(
                passport.validate_expiration_year(),
                true,
                "expiration year: {:?}",
                &passport.eyr
            );
        }
        for invalid_year in invalid_expiration_years {
            passport.eyr = Some(invalid_year);
            assert_eq!(
                passport.validate_expiration_year(),
                false,
                "expiration year: {:?}",
                &passport.eyr
            );
        }
    }

    #[test]
    fn test_passport_validate_height() {
        let mut passport = Passport::empty();
        let valid_heights = vec![
            "150cm".to_string(),
            "193cm".to_string(),
            "59in".to_string(),
            "76in".to_string(),
        ];
        let invalid_heights = vec![
            "2031".to_string(),
            "150in".to_string(),
            "194cm".to_string(),
            "76cm".to_string(),
            "cm".to_string(),
            "190".to_string(),
            "0190cm".to_string(),
            "060in".to_string(),
            "0.60in".to_string(),
            "60.0in".to_string(),
        ];

        for height in valid_heights {
            passport.hgt = Some(height);
            assert_eq!(
                passport.validate_height(),
                true,
                "height: {:?}",
                &passport.hgt
            );
        }
        for invalid_height in invalid_heights {
            passport.hgt = Some(invalid_height);
            assert_eq!(
                passport.validate_height(),
                false,
                "height: {:?}",
                &passport.hgt
            );
        }
    }

    #[test]
    fn test_passport_validate_hair_color() {
        let mut passport = Passport::empty();
        let valid_hair_colors = vec![
            "#123abc".to_string(),
            "#ffffff".to_string(),
            "#000000".to_string(),
            "#0faf00".to_string(),
        ];
        let invalid_hair_colors = vec![
            "2031".to_string(),
            "0faf000".to_string(),
            "#123abz".to_string(),
            "######".to_string(),
            "#0000000".to_string(),
            "".to_string(),
        ];

        for color in valid_hair_colors {
            passport.hcl = Some(color);
            assert_eq!(
                passport.validate_hair_color(),
                true,
                "hair color: {:?}",
                &passport.hcl
            );
        }
        for invalid_color in invalid_hair_colors {
            passport.hcl = Some(invalid_color);
            assert_eq!(
                passport.validate_hair_color(),
                false,
                "hair color: {:?}",
                &passport.hcl
            );
        }
    }

    #[test]
    fn test_passport_validate_eye_color() {
        let mut passport = Passport::empty();
        let valid_eye_colors = vec![
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string(),
        ];
        let invalid_eye_colors = vec![
            "ambb".to_string(),
            "2031".to_string(),
            "".to_string(),
            "o".to_string(),
            "hz".to_string(),
            "\n".to_string(),
        ];
        for color in valid_eye_colors {
            passport.ecl = Some(color);
            assert_eq!(
                passport.validate_eye_color(),
                true,
                "eye color: {:?}",
                &passport.ecl
            );
        }
        for invalid_color in invalid_eye_colors {
            passport.ecl = Some(invalid_color);
            assert_eq!(
                passport.validate_eye_color(),
                false,
                "eye color: {:?}",
                &passport.ecl
            );
        }
    }

    #[test]
    fn test_passport_validate_passport_id() {
        let mut passport = Passport::empty();
        let valid_passport_ids = vec![
            "000000000".to_string(),
            "999999999".to_string(),
            "900000000".to_string(),
            "000000009".to_string(),
            "123456789".to_string(),
            "010101010".to_string(),
            "076543210".to_string(),
        ];
        let invalid_passport_ids = vec![
            "00000000".to_string(),   // only 8 digits
            "0000000000".to_string(), // 10 digits
            "".to_string(),
            "o".to_string(),
            "00a000000".to_string(),
            "99999999z".to_string(),
            "a11111111".to_string(),
            "\n".to_string(),
        ];
        for passport_id in valid_passport_ids {
            passport.pid = Some(passport_id);
            assert_eq!(
                passport.validate_passport_id(),
                true,
                "passport id: {:?}",
                &passport.pid
            );
        }
        for invalid_passport_id in invalid_passport_ids {
            passport.pid = Some(invalid_passport_id);
            assert_eq!(
                passport.validate_passport_id(),
                false,
                "passport id: {:?}",
                &passport.pid
            );
        }
    }

    #[test]
    fn test_passport_validate() {
        let valid_input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
            hcl:#623a2f\n\
            \n\
            eyr:2029 ecl:blu cid:129 byr:1989\n\
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
            \n\
            hcl:#888785\n\
            hgt:164cm byr:2001 iyr:2015 cid:88\n\
            pid:545766238 ecl:hzl\n\
            eyr:2022\n\
            \n\
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n\
            \n";

        let invalid_input = "eyr:1972 cid:100\n\
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
            \n\
            iyr:2019\n\
            hcl:#602927 eyr:1967 hgt:170cm\n\
            ecl:grn pid:012533040 byr:1946\n\
            \n\
            hcl:dab227 iyr:2012\n\
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
            \n\
            hgt:59cm ecl:zzz\n\
            eyr:2038 hcl:74454a iyr:2023\n\
            pid:3556412378 byr:2007\n";

        let valid_passports = parse_passports(valid_input);
        let invalid_passports = parse_passports(invalid_input);

        for valid_passport in valid_passports {
            assert_eq!(
                valid_passport.validate(),
                true,
                "passport: {:?}",
                &valid_passport
            );
        }
        for invalid_passport in invalid_passports {
            assert_eq!(
                invalid_passport.validate(),
                false,
                "passport: {:?}",
                &invalid_passport
            );
        }
    }

    #[test]
    fn test_part_2() {
        let mut input = String::new();
        let valid_input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
        hcl:#623a2f\n\
        \n\
        eyr:2029 ecl:blu cid:129 byr:1989\n\
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
        \n\
        hcl:#888785\n\
        hgt:164cm byr:2001 iyr:2015 cid:88\n\
        pid:545766238 ecl:hzl\n\
        eyr:2022\n\
        \n\
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n\
        \n";

        let invalid_input = "eyr:1972 cid:100\n\
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
        \n\
        iyr:2019\n\
        hcl:#602927 eyr:1967 hgt:170cm\n\
        ecl:grn pid:012533040 byr:1946\n\
        \n\
        hcl:dab227 iyr:2012\n\
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
        \n\
        hgt:59cm ecl:zzz\n\
        eyr:2038 hcl:74454a iyr:2023\n\
        pid:3556412378 byr:2007\n";

        input.push_str(valid_input);
        input.push_str(invalid_input);
        assert_eq!(part2(input.as_str()), 4);
    }
}
