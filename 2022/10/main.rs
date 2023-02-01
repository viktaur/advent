use std::fs;

fn part_one(input: &str) -> Option<i32>  {
    let mut register_val = 1;
    let mut cycle = 1;
    let mut signal_strengths_sum = 0;

    for line in input.lines() {
        if line.starts_with("addx") {
            let increase_val: i32 = line.replace("addx ", "").parse::<i32>().unwrap();

            let mut cycles_to_complete = 2;

            while cycles_to_complete > 0 {
                if cycle % 40 == 20 {
                    signal_strengths_sum += cycle * register_val;
                }

                cycles_to_complete -= 1;
                cycle += 1;
            }

            register_val += increase_val;
        } else {
            if cycle % 40 == 20 {
                signal_strengths_sum += cycle * register_val;
            }

            cycle += 1;
        }
    }

    Some(signal_strengths_sum)
}

fn pixel_under_sprite(pixel_pos: i32, sprite_pos: i32) -> bool {
    pixel_pos >= (sprite_pos - 1) && pixel_pos <= (sprite_pos + 1)
}

fn part_two(input: &str) -> Option<String> {
    let mut result: String = String::new();
    let mut row_pixels: String = String::new();

    let mut register_val = 1;
    let mut pos = 0;

    for line in input.lines() {
        if line.starts_with("addx") {
            let increase_val: i32 = line.replace("addx ", "").parse::<i32>().unwrap();

            let mut cycles_to_complete = 2;

            while cycles_to_complete > 0 {

                if pos == 40 {
                    result.push_str(&row_pixels.clone());
                    result.push('\n');
                    row_pixels = String::new();
                    pos = 0;
                }

                if pixel_under_sprite(pos, register_val) {
                    row_pixels.push('#');
                } else {
                    row_pixels.push('.');
                }

                cycles_to_complete -= 1;
                pos += 1;
            }

            register_val += increase_val;
        } else {

            if pos == 40 {
                result.push_str(&row_pixels.clone());
                result.push('\n');
                row_pixels = String::new();
                pos = 0;
            }

            if pixel_under_sprite(pos, register_val) {
                row_pixels.push('#');
            } else {
                row_pixels.push('.');
            }

            pos += 1;
        }
    }

    result.push_str(&row_pixels.clone());

    Some(result)
}


fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file");
    let result1 = part_one(&input).unwrap();
    let result2 = part_two(&input).unwrap();
    println!("{result1}");
    println!("{result2}");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("src/test.txt").expect("Error reading file");
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("src/test.txt").expect("Error reading file");
        let result = part_two(&input).unwrap();
        println!("{result}");
    }
}

