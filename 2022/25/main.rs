use std::fs;
use std::thread::sleep;

fn mod_to_snafu(mod_digit: i64) -> &'static str {
    match mod_digit {
        0 => "00",
        1 => "01",
        2 => "02",
        3 => "1=",
        4 => "1-",
        _ => "00"
    }
}

fn add_snafu_digits(s1: char, s2: char, carry: i64) -> (char, i64) {
    let snafu_val = snafu_to_decimal(&s1.to_string()) + snafu_to_decimal(&s2.to_string()) + carry;
    // let options: Vec<(char, i64)> = vec![('0', 0), ('1', 0), ('2', 0), ('=', 1), ('-', 1)];
    let options_char: Vec<char> = vec!['=', '-', '0', '1', '2'];
    let selected_char = options_char.get(((snafu_val+2).rem_euclid(5)) as usize).unwrap();

    if snafu_val >= 3 {
        return (*selected_char, 1);
    }

    (*selected_char, 0)
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut num = decimal;
    let mut snafu = String::new();
    let mut snafus_vec: Vec<String>= vec![];

    let mut i: usize = 0;

    while num > 0 {
        let q: i64 = num / 5;
        let r: i64 = num - (q * 5);

        snafus_vec.push(mod_to_snafu(r).to_string());
        num = q;
        i += 1;
    }


    let mut carry = 0;
    let mut j: usize = 0;

    snafu.push(snafus_vec.get(0).unwrap().chars().nth(1).unwrap());

    while j < (i - 1) {
        let s1 = snafus_vec.get(j).unwrap().chars().nth(0).unwrap();
        let s2 = snafus_vec.get(j+1).unwrap().chars().nth(1).unwrap();

        let (digit, new_carry) = add_snafu_digits(s1, s2, carry);
        snafu.push(digit);
        carry = new_carry;
        j += 1;
    }

    // snafu.push(snafus_vec[j][0]);
    let start = snafus_vec.get(j).unwrap().chars().nth(0).unwrap();

    if start != '0' {
        snafu.push(start);
    }

    snafu.chars().rev().collect()
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut decimal: i64 = 0;

    for (pos, c) in snafu.chars().rev().enumerate() {
        let val: i64 = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => 0
        };

        decimal += 5_i64.pow(pos as u32) * val;
    }

    decimal
}

fn part_one(input: &str) -> Option<String> {

    let mut sum_as_decimal = 0;

    for l in input.lines() {
        sum_as_decimal += snafu_to_decimal(l);
    }

    Some(decimal_to_snafu(sum_as_decimal))
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file input.txt");
    println!("{}", part_one(&input).unwrap())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("src/sample.txt").expect("Error reading file text.txt for part one");
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("src/sample.txt").expect("Error reading file text.txt for part one");
        // assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1=0"), 15);
        assert_eq!(snafu_to_decimal("1=11-2"), 2022);
        assert_eq!(snafu_to_decimal("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(10), String::from("20"));
        assert_eq!(decimal_to_snafu(906), String::from("12111"));
        assert_eq!(decimal_to_snafu(1257), String::from("20012"))
    }
}
