use std::{io::{BufReader, BufRead}, fs::File};

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let file_lines = in_file.lines().collect::<Vec<Result<String, std::io::Error>>>();

    let mut lines = Vec::new();
    for item in file_lines.into_iter() {
        lines.push(item?);
    }

    let stacks_instructions = lines.split(|x| x.is_empty()).collect::<Vec<&[String]>>();
    let stacks = stacks_instructions[0];
    let instructions = stacks_instructions[1];

    let stack_list = stacks.iter().rev().map(|x| x.split(' ').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    for stack_row in stack_list {
        for (idx, stack_entry) in stack_row.iter().enumerate() {
            println!("idx: {}, stack: {}", idx, stack_entry);
        }
    }

    Ok(())
}