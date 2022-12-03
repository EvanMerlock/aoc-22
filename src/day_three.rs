use std::{io::{BufReader, BufRead}, fs::File};

fn char_to_priority(c: char) -> u64 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("bad char"),
    }
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let file_lines = in_file.lines().collect::<Vec<Result<String, std::io::Error>>>();

    let mut lines = Vec::new();
    for item in file_lines.into_iter() {
        lines.push(item?);
    }

    let rucksacks = lines
        .into_iter()
        .map(|mut x| {
            let len = x.len();
            let half = x.split_off(len/2);
            (x, half)
        })
        .collect::<Vec<(String, String)>>();

    let ruck_matches = rucksacks
        .into_iter()
        .map(|(ruck1, ruck2)| {
            let mut ruck1_bin: u64 = 0;
            let mut ruck2_bin: u64 = 0;

            for i in ruck1.chars() {
                ruck1_bin = ruck1_bin | (1 << char_to_priority(i));
            }

            for i in ruck2.chars() {
                ruck2_bin = ruck2_bin | (1 << char_to_priority(i));
            }

            ruck1_bin & ruck2_bin
        }).collect::<Vec<u64>>();

    let priority = ruck_matches
        .into_iter()
        .map(|x| {
            let mut prio: u64 = 0;
            for i in 0..64 {
                if ((x & (1 << i)) >> i) == 1 {
                    prio = prio + i;
                }
            }
            prio
        }).reduce(|acc, prio| acc + prio).expect("failed to tabulate priority");

    println!("Priority Sum: {}", priority);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let file_lines = in_file.lines().collect::<Vec<Result<String, std::io::Error>>>();

    let mut lines = Vec::new();
    for item in file_lines.into_iter() {
        lines.push(item?);
    }

    let rucksacks = lines
        .into_iter()
        .map(|ruck| {
            let mut ruck_bin: u64 = 0;

            for i in ruck.chars() {
                ruck_bin = ruck_bin | (1 << char_to_priority(i));
            }

            ruck_bin
        }).collect::<Vec<u64>>();
    
    let merged_ruck_sacks = rucksacks
        .chunks_exact(3)
        .map(|x| {
            x[0] & x[1] & x[2]
        }).collect::<Vec<u64>>();
    
    let priority = merged_ruck_sacks
        .into_iter()
        .map(|mruck| {
            let mut prio: u64 = 0;
            for i in 0..64 {
                if ((mruck & (1 << i)) >> i) == 1 {
                    prio = prio + i;
                }
            }
            prio
        }).reduce(|acc, x| acc + x).expect("failed to reduce prio");

    println!("Priority: {}", priority);

    Ok(())
}