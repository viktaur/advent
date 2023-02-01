use std::collections::{HashMap, VecDeque};
use std::fs;

enum Dir {
    N,
    S,
    W,
    E,
}

fn can_move_dir(dir: &Dir, &x: &i32, &y: &i32, coordinates: &HashMap<(i32, i32), char>) -> bool {
    match dir {
        Dir::N => [x-1, x, x+1].iter().all(|&c| *coordinates.get(&(c, y-1)).unwrap_or(&'.') == '.'),
        Dir::S => [x-1, x, x+1].iter().all(|&c| *coordinates.get(&(c, y+1)).unwrap_or(&'.') == '.'),
        Dir::W => [y-1, y, y+1].iter().all(|&c| *coordinates.get(&(x-1, c)).unwrap_or(&'.') == '.'),
        Dir::E => [y-1, y, y+1].iter().all(|&c| *coordinates.get(&(x+1, c)).unwrap_or(&'.') == '.'),
    }
}

fn elf_is_alone(&x: &i32, &y: &i32, coordinates: &HashMap<(i32, i32), char>) -> bool {
    [
        (x-1, y-1), (x, y-1), (x+1, y-1),
        (x-1, y), (x+1, y),
        (x-1, y+1), (x, y+1), (x+1, y+1)
    ].iter().all(|c| *coordinates.get(c).unwrap_or(&'.') == '.')
}

fn show(round: i32, coordinates: &HashMap<(i32, i32), char>) {
    let max_N = coordinates.keys().min_by(|&k1, &k2| k1.1.cmp(&k2.1)).expect("Could not find max N").1;
    let max_S = coordinates.keys().max_by(|&k1, &k2| k1.1.cmp(&k2.1)).expect("Could not find max S").1;
    let max_W = coordinates.keys().min_by(|&k1, &k2| k1.0.cmp(&k2.0)).expect("Could not find max W").0;
    let max_E = coordinates.keys().max_by(|&k1, &k2| k1.0.cmp(&k2.0)).expect("Could not find max E").0;

    if round == 0 {
        println!("=== INITIAL STATE ===");
    }

    for i in max_N..(max_S+1) {
        for j in max_W..(max_E+1) {
            print!("{}", coordinates.get(&(j, i)).unwrap_or(&'.'))
        }
        println!();
    }

    println!("=== END OF ROUND {} ===", round+1);
    println!();
    println!();
}

fn part_one(input: &str) -> Option<i32> {
    let mut coordinates: HashMap<(i32, i32), char> = HashMap::new();
    let mut directions_queue: VecDeque<Dir> = VecDeque::from([Dir::N, Dir::S, Dir::W, Dir::E]);
    let (mut max_N, mut max_S, mut max_W, mut max_E) = (0, 0, 0, 0);

    // parse
    for (i, line) in input.lines().enumerate() {
        for (j, val) in line.chars().into_iter().enumerate() {
            coordinates.insert((j as i32, i as i32), val);
        }
    }

    // show(0, &coordinates);

    // for each round
    for round in 0..10 {

        // first half
        let mut next_move_elf: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut coordinate_counter: HashMap<(i32, i32), i32> = HashMap::new();

        for ((x, y), val) in coordinates.iter() {
            if *val == '#' && !elf_is_alone(x, y, &coordinates) {
                for i in 0..4 {
                    if can_move_dir(directions_queue.get(i).unwrap(), x, y, &coordinates) {
                        let (xp, yp) = match directions_queue.get(i).unwrap() {
                            Dir::N => (*x, *y - 1),
                            Dir::S => (*x, *y + 1),
                            Dir::W => (*x - 1, *y),
                            Dir::E => (*x + 1, *y)
                        };

                        next_move_elf.insert((*x, *y), (xp, yp));
                        *coordinate_counter.entry((xp, yp)).or_insert(0) += 1;

                        break;
                    }
                }
            }
        }

        // second half
        for ((x, y), (xp, yp)) in next_move_elf.iter() {
            if *coordinate_counter.get(&(*xp, *yp)).unwrap() == 1 {
                coordinates.insert((*xp, *yp), '#');
                coordinates.insert((*x, *y), '.');
            }
        }

        // after
        if let Some(first) = directions_queue.pop_front() {
            directions_queue.push_back(first);
        }

        //logging
        // show(round, &coordinates);
    }

    let mut n_empty_ground_tiles = 0;

    // AFTER ROUND 10

    // find boundaries
    max_N = coordinates.keys().min_by(|&k1, &k2| k1.1.cmp(&k2.1)).expect("Could not find max N").1;
    max_S = coordinates.keys().max_by(|&k1, &k2| k1.1.cmp(&k2.1)).expect("Could not find max S").1;
    max_W = coordinates.keys().min_by(|&k1, &k2| k1.0.cmp(&k2.0)).expect("Could not find max W").0;
    max_E = coordinates.keys().max_by(|&k1, &k2| k1.0.cmp(&k2.0)).expect("Could not find max E").0;

    for i in max_N..(max_S+1) {
        for j in max_W..(max_E+1) {
            if *coordinates.get(&(j, i)).unwrap_or(&'.') == '.' {
                n_empty_ground_tiles += 1;
            }
        }
    }

    Some(n_empty_ground_tiles)
}

fn part_two(input: &str) -> Option<i32> {
    let mut coordinates: HashMap<(i32, i32), char> = HashMap::new();
    let mut directions_queue: VecDeque<Dir> = VecDeque::from([Dir::N, Dir::S, Dir::W, Dir::E]);
    let (mut max_N, mut max_S, mut max_W, mut max_E) = (0, 0, 0, 0);

    // parse
    for (i, line) in input.lines().enumerate() {
        for (j, val) in line.chars().into_iter().enumerate() {
            coordinates.insert((j as i32, i as i32), val);
        }
    }

    let mut round = 0;
    let mut elves_move = true;

    // for each round
    while elves_move {
        elves_move = false;

        // first half
        let mut next_move_elf: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut coordinate_counter: HashMap<(i32, i32), i32> = HashMap::new();

        for ((x, y), val) in coordinates.iter() {
            if *val == '#' && !elf_is_alone(x, y, &coordinates) {
                for i in 0..4 {
                    if can_move_dir(directions_queue.get(i).unwrap(), x, y, &coordinates) {
                        let (xp, yp) = match directions_queue.get(i).unwrap() {
                            Dir::N => (*x, *y - 1),
                            Dir::S => (*x, *y + 1),
                            Dir::W => (*x - 1, *y),
                            Dir::E => (*x + 1, *y)
                        };

                        next_move_elf.insert((*x, *y), (xp, yp));
                        *coordinate_counter.entry((xp, yp)).or_insert(0) += 1;

                        elves_move = true;
                        break;
                    }
                }
            }
        }

        // second half
        for ((x, y), (xp, yp)) in next_move_elf.iter() {
            if *coordinate_counter.get(&(*xp, *yp)).unwrap() == 1 {
                coordinates.insert((*xp, *yp), '#');
                coordinates.insert((*x, *y), '.');
            }
        }

        // after
        if let Some(first) = directions_queue.pop_front() {
            directions_queue.push_back(first);
        }

        //logging
        // show(round, &coordinates);

        round += 1;
    }
    Some(round)
}


fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    println!("{}", part_one(&input).unwrap());
    println!("{}", part_two(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("src/sample.txt").unwrap();
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("src/sample.txt").unwrap();
        assert_eq!(part_two(&input), Some(20));
    }
}
