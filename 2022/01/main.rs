use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::BinaryHeap;

#[derive(Ord, Eq, PartialEq, PartialOrd)]
struct Elf {
    calories: i32
}

impl Elf {
    fn new (calories: i32) -> Elf {
        Elf {calories}
    }

    fn add (&mut self, cal: i32) {
        self.calories += cal;
    }
}

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let mut elves: BinaryHeap<Elf> = BinaryHeap::new();
    let mut current_elf: Elf = Elf::new(0);

    for line in BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                // content.push(line)
                let cals = match line.parse() {
                    Ok(val) => val,
                    Err(_) => 0
                };

                current_elf.add(cals);

                if line == "" {
                    elves.push(current_elf);
                    current_elf = Elf::new(0);
                }
            },

            Err(_) => {
                break;
            }
        }
    }

    // match elves.peek() {
    //     Some(e) => print!("{}", e.calories),
    //     None => print!("{}", 0)
    // }

    let mut sum = 0;

    for _i in 0..3 {
        match elves.pop() {
            Some(e) => { sum += e.calories },
            None => { sum += 0 }
        }
    }

    print!("{}", sum)
}
