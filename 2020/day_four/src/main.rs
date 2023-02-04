use std::fs;
use prse::parse;
use regex::Regex;

fn part_one(input: &str) -> Option<i32> {

    let mut valid_passports = 0;

    let mut n_fields = 0;
    let mut contains_cid = false;

    for line in input.lines() {
        if line == "" {
            if (n_fields == 8) || (n_fields == 7 && !contains_cid) {
                valid_passports += 1;
                println!("valid\n");
            } else {
                println!("invalid\n")
            }
        
            // println!("{valid_passports}");

            n_fields = 0;
            contains_cid = false;
        } else {
            let fields = line.split(' ');

            for f in fields {
                if f.split(':').collect::<Vec<&str>>()[0] == "cid" {
                    contains_cid = true;
                }

                println!("{f} {n_fields} {contains_cid}");

                n_fields += 1;
            }
        }
    }

    if (n_fields == 8) || (n_fields == 7 && !contains_cid) {
        valid_passports += 1;
        println!("valid\n");
    } else {
        println!("invalid\n")
    }

    Some(valid_passports)
}

fn valid_field(code: &str, val: &str) -> bool {
    match code {
        "byr" => val.parse::<i32>().unwrap() >= 1920 && val.parse::<i32>().unwrap() <= 2002,
        "iyr" => val.parse::<i32>().unwrap() >= 2010 && val.parse::<i32>().unwrap() <= 2020,
        "eyr" => val.parse::<i32>().unwrap() >= 2020 && val.parse::<i32>().unwrap() <= 2030,
        "hgt" => {
            if val.contains("cm") {
                let num: i32 = parse!(val, "{}cm");
                num >= 150 && num <= 193
            } else if val.contains("in") {
                let num: i32 = parse!(val, "{}in");
                num >= 59 && num <= 76
            } else {
                false
            }
        },
        "hcl" => {
            val.starts_with('#') && {
                let re = Regex::new(r"([0-9a-f]{6})").expect("Error in pattern re");

                re.is_match(val)
            }
        },
        "ecl" => {
            match val {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false
            }
        },
        "pid" => {
            let re = Regex::new(r"^\d{9}$").expect("Error in pattern re");
            re.is_match(val)
        },
        _ => false,
    }
}

fn part_two(input: &str) -> Option<i32> {

    let mut valid_passports = 0;

    // we will update these two fields for every passport (set of fields) we read
    let mut passport_is_valid = true;
    let mut n_fields = 0;

    for line in input.lines() {
        if line == "" {
            if n_fields == 7 {
                valid_passports += 1;
            }
        
            // reset for the next passport
            passport_is_valid = true;
            n_fields = 0;

        } else if passport_is_valid {
            let fields = line.split(" ");

            for f in fields {
                let (code, val): (&str, &str) = parse!(f, "{}:{}");

                if code != "cid" {
                    if valid_field(code, val) {
                        n_fields += 1;
                    } else {
                        passport_is_valid = false;
                        break;
                    }
                }
            }
        }
    }

    if n_fields == 7 {
        valid_passports += 1;
    }

    Some(valid_passports)
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file");
    println!("{}", part_one(&input).unwrap());
    println!("{}", part_two(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("src/example.txt").expect("Error reading file");
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_valid_field() {
        assert_eq!(valid_field("byr", "2002"), true);
        assert_eq!(valid_field("byr", "2003"), false);

        assert_eq!(valid_field("hgt", "60in"), true);
        assert_eq!(valid_field("hgt", "190cm"), true);
        assert_eq!(valid_field("hgt", "190in"), false);
        assert_eq!(valid_field("hgt", "190"), false);

        assert_eq!(valid_field("hcl", "#123abc"), true);
        assert_eq!(valid_field("hcl", "#123abz"), false);
        assert_eq!(valid_field("hcl", "123abc"), false);

        assert_eq!(valid_field("ecl", "brn"), true);
        assert_eq!(valid_field("ecl", "wat"), false);

        assert_eq!(valid_field("pid", "000000001"), true);
        assert_eq!(valid_field("pid", "0123456789"), false);
    }

    #[test]
    fn test_part_two() {
        let valid_passports = fs::read_to_string("src/valid_passports.txt").expect("Error reading file");
        let invalid_passports = fs::read_to_string("src/invalid_passports.txt").expect("Error reading file");

        assert_eq!(part_two(&valid_passports), Some(4));
        assert_eq!(part_two(&invalid_passports), Some(0));
    }
}

