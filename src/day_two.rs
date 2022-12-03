use std::{io::{BufReader, BufRead}, fs::File};

enum RPS {
    Rock,
    Paper,
    Scissors
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        match c {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,

            'X' => RPS::Rock,
            'Y' => RPS::Paper,
            'Z' => RPS::Scissors,

            _ => panic!("bad char")
        }
    }
}

fn generate_score(games: Vec<(RPS, RPS)>) -> i32 {
    games.into_iter().map(|x| {
        match x {
            (RPS::Rock, RPS::Rock) => 1 + 3,
            (RPS::Paper, RPS::Paper) => 2 + 3,
            (RPS::Scissors, RPS::Scissors) => 3 + 3,

            (RPS::Rock, RPS::Paper) => 2 + 6,
            (RPS::Rock, RPS::Scissors) => 3 + 0,

            (RPS::Paper, RPS::Rock) => 1 + 0,
            (RPS::Paper, RPS::Scissors) => 3 + 6,

            (RPS::Scissors, RPS::Rock) => 1 + 6,
            (RPS::Scissors, RPS::Paper) => 2 + 0,
        }
    }).reduce(|acc, x| acc+x).expect("failed to reduce")
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let file_lines = in_file.lines().collect::<Vec<Result<String, std::io::Error>>>();

    let mut lines = Vec::new();
    for item in file_lines.into_iter() {
        lines.push(item?);
    }

    let games = lines
        .into_iter()
        .map(|x| {
            let opp = x.chars().nth(0).expect("failed to extract opp move");
            let me = x.chars().nth(2).expect("failed to extract me move");

            (RPS::from(opp), RPS::from(me))
        }).collect::<Vec<(RPS, RPS)>>();

    let results = generate_score(games);

    println!("final score: {}", results);


    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let file_lines = in_file.lines().collect::<Vec<Result<String, std::io::Error>>>();

    let mut lines = Vec::new();
    for item in file_lines.into_iter() {
        lines.push(item?);
    }

    let games = lines
        .into_iter()
        .map(|x| {
            let opp = x.chars().nth(0).expect("failed to extract opp move");
            let me = x.chars().nth(2).expect("failed to extract me move");

            let opp_move = RPS::from(opp);

            let me_move = match (&opp_move, me) {
                (RPS::Rock, 'X') => RPS::Scissors,
                (RPS::Rock, 'Y') => RPS::Rock,
                (RPS::Rock, 'Z') => RPS::Paper,

                (RPS::Paper, 'X') => RPS::Rock,
                (RPS::Paper, 'Y') => RPS::Paper,
                (RPS::Paper, 'Z') => RPS::Scissors,

                (RPS::Scissors, 'X') => RPS::Paper,
                (RPS::Scissors, 'Y') => RPS::Scissors,
                (RPS::Scissors, 'Z') => RPS::Rock,

                _ => panic!("invalid char")
            };

            (opp_move, me_move)
        }).collect::<Vec<(RPS, RPS)>>();


        
    let results = generate_score(games);

    println!("final score: {}", results);

    Ok(())
}