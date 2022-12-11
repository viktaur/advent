use std::fs;
use std::str::FromStr;

fn parse(p: &str) -> Option<(i32, i32, i32, i32)> {
    let (p1, p2) = p.split_once(',')?;

    let (p1_s, p1_e) = (i32::from_str(p1.split_once('-')?.0).ok()?, i32::from_str(p1.split_once('-')?.1).ok()?);
    let (p2_s, p2_e) = (i32::from_str(p2.split_once('-')?.0).ok()?, i32::from_str(p2.split_once('-')?.1).ok()?);

    Some((p1_s, p1_e, p2_s, p2_e))
}

fn part_one(input: &String) -> Option<u32> {
    
    let mut pairs = 0;

    for p in input.lines() {

        let (p1_s, p1_e, p2_s, p2_e) = parse(&p)?;

        if (p1_s <= p2_s && p2_e <= p1_e) || (p2_s <= p1_s && p1_e <= p2_e) {
            pairs += 1;
        }
    }
    
    Some(pairs)
}

fn part_two(input: &String) -> Option<u32> {

    let mut pairs = 0;

    for p in input.lines() {

        let (p1_s, p1_e, p2_s, p2_e) = parse(&p)?;

        if (p2_s >= p1_s && p2_s <= p1_e) || (p1_s >= p2_s && p1_s <= p2_e) {
            pairs += 1;
        }
    }

    Some(pairs)
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file");

    let one = part_one(&input).unwrap();
    println!("{one}");

    let two = part_two(&input).unwrap();
    println!("{two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input1 = fs::read_to_string("src/1.txt").expect("Error reading file");
        assert_eq!(part_one(&input1), Some(2));

        let input2 = fs::read_to_string("src/my1.txt").expect("Error reading file");
        assert_eq!(part_one(&input2), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("src/1.txt").expect("Error reading file");
        assert_eq!(part_two(&input), Some(4));
    }
}
