use std::fs::File;
use std::io::{BufReader,BufRead};

#[derive(Debug)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

impl Selection {
    fn points(&self) -> i32 {
        match self {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3,
        }
    }
}

enum Outcome {
    Lost,
    Draw,
    Won,
}

impl Outcome {
    fn points(&self) -> i32 {
        match self {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Won => 6,
        }
    }
}

fn get_sel_from_code(code: &str) -> Option<Selection> {
    match code {
        "A" | "X" => Some(Selection::Rock),
        "B" | "Y" => Some(Selection::Paper),
        "C" | "Z" => Some(Selection::Scissors),
        _ => None,
    }
}

fn get_out_from_code(code: &str) -> Option<Outcome> {
    match code {
        "X" => Some(Outcome::Lost),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Won),
        _ => None,
    }
}

fn calc_points_round(op: &Selection, us: &Selection) -> i32 {
    let out: Outcome = match (op, us) {
        (Selection::Rock, Selection::Rock) => Outcome::Draw,
        (Selection::Rock, Selection::Paper) => Outcome::Won,
        (Selection::Rock, Selection::Scissors) => Outcome::Lost,
        (Selection::Paper, Selection::Rock) => Outcome::Lost,
        (Selection::Paper, Selection::Paper) => Outcome::Draw,
        (Selection::Paper, Selection::Scissors) => Outcome::Won,
        (Selection::Scissors, Selection::Rock) => Outcome::Won,
        (Selection::Scissors, Selection::Paper) => Outcome::Lost,
        (Selection::Scissors, Selection::Scissors) => Outcome::Draw,
    };

    us.points() + out.points()
}

fn det_sel(op: &Selection, out: &Outcome) -> Selection {
    match (op, out) {
        (Selection::Rock, Outcome::Won) => Selection::Paper,
        (Selection::Rock, Outcome::Draw) => Selection::Rock,
        (Selection::Rock, Outcome::Lost) => Selection::Scissors,
        (Selection::Paper, Outcome::Won) => Selection::Scissors,
        (Selection::Paper, Outcome::Draw) => Selection::Paper,
        (Selection::Paper, Outcome::Lost) => Selection::Rock,
        (Selection::Scissors, Outcome::Won) => Selection::Rock,
        (Selection::Scissors, Outcome::Draw) => Selection::Scissors,
        (Selection::Scissors, Outcome::Lost) => Selection::Paper,
    }
}

fn part_one(file: &File) {

    let mut total_score = 0;

    for line in BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                let elems: Vec<&str> = line.split_whitespace().collect();
                let op: Selection = get_sel_from_code(elems.get(0).unwrap()).unwrap();
                let us: Selection = get_sel_from_code(elems.get(1).unwrap()).unwrap();

                total_score += calc_points_round(&op, &us);
            },

            Err(_) => {
                break;
            }
        }
    }

    println!("{}", total_score)
}

fn part_two(file: &File) {
    let mut total_score = 0;

    for line in BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                let elems: Vec<&str> = line.split_whitespace().collect();
                let op: Selection = get_sel_from_code(elems.get(0).unwrap()).unwrap();
                let out: Outcome = get_out_from_code(elems.get(1).unwrap()).unwrap();

                let us: Selection = det_sel(&op, &out);


                total_score += calc_points_round(&op, &us);
            },

            Err(_) => {
                break;
            }
        }
    }

    println!("{}", total_score)
}

fn main() {
    let file1 = File::open("src/input.txt").unwrap();
    part_one(&file1);

    let file2 = File::open("src/input.txt").unwrap();
    part_two(&file2);
}
