use std::{io::{BufReader, BufRead}, fs::File};

fn parse_elves(in_file: BufReader<File>) -> Result<Vec<usize>, std::io::Error> {
    let file_lines = in_file.lines().collect::<Vec<Result<String, std::io::Error>>>();

    let mut lines = Vec::new();
    for item in file_lines.into_iter() {
        lines.push(item?);
    }

    let mut separations: Vec<usize> = Vec::new();

    for (idx, item) in lines.iter().enumerate() {
        if item == "" {
            // mark as separation
            separations.push(idx);
        }
    }

    let mut elves: Vec<usize> = Vec::new();
    let mut last_separation = 0;
    for item in separations {
        let slice = &lines[last_separation+1..item];
        elves.push(
            slice
                .iter()
                .map(|calorie| calorie.parse::<usize>().expect("failed to parse elf"))
                .reduce(|accum, item| accum + item).expect("failed to compute elf")
        );
        last_separation = item;
    }

    Ok(elves)
}

fn find_elf(elves: &mut Vec<usize>) -> (usize, usize) {
    let elf = elves.iter().enumerate().map(|x| (x.0, x.1.clone())).reduce(|accum, item| if item.1 > accum.1 { item } else { accum }).expect("failed to reduce");
    elves.remove(elf.0);
    (elf.0, elf.1)
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let mut elves = parse_elves(in_file)?;
    let elf = find_elf(&mut elves);
    println!("Elf: #{}, with {} calories", elf.0, elf.1);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let mut elves = parse_elves(in_file)?;

    let elf1 = find_elf(&mut elves);
    let elf2 = find_elf(&mut elves);
    let elf3 = find_elf(&mut elves);

    println!("Elf 1: #{}, with {} calories", elf1.0, elf1.1);
    println!("Elf 2: #{}, with {} calories", elf2.0, elf2.1);
    println!("Elf 3: #{}, with {} calories", elf3.0, elf3.1);
    println!("Elf Summation: {}", elf1.1 + elf2.1 + elf3.1);

    Ok(())
}