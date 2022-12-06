use core::panic;
use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

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
    let stacks_total = stack_list[0].iter().filter(|x| !x.is_empty()).collect::<Vec<&&str>>();
    println!("total stacks: {:?}", stacks_total);

    let mut stacks: HashMap<String, Vec<String>> = HashMap::new();
    for item in stacks_total{
        stacks.insert(item.to_string(), Vec::new());
    }

    for stack_row in &stack_list[1..] {
        let mut curr_stack = 1;
        let mut curr_loc = 0;
        while curr_loc < stack_row.len() {
            let stack_entry = stack_row.get(curr_loc).expect("failed to get stack entry");
            if !stack_entry.is_empty() {
                stacks.get_mut(&curr_stack.to_string()).expect("failed to get stack").push(stack_entry.to_string());
            } else {
                curr_loc += 3;
            }
            curr_loc += 1;
            curr_stack += 1;
        }
    }

    println!("{:?}", stacks);

    for instruction in instructions {
        let mut move_f = false;
        let mut from = false;
        let mut to = false;

        let mut inst_num = 0;
        let mut from_num = 0;
        let mut to_num = 0;
        for item in instruction.split(' ') {
            match item {
                "move" => move_f = true,
                "from" => from = true,
                "to" => to = true,
                other if move_f && !from => {
                    inst_num = other.parse().expect("failed to parse inst num");
                },
                other if from  && !to => {
                    from_num = other.parse().expect("failed to parse from num");
                },
                other if to => {
                    to_num = other.parse().expect("failed to parse to num");

                },
                _ => panic!("bad")
            }
        }

        for _ in 0..inst_num {
            let stack_val = stacks.get_mut(&from_num.to_string()).expect("failed to get stack").pop().expect("failed to pop from stack");
            stacks.get_mut(&to_num.to_string()).expect("failed to get stack").push(stack_val);
        }
    }

    println!("{:?}", stacks);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let file_lines = in_file.lines().collect::<Vec<Result<String, std::io::Error>>>();

    let mut lines = Vec::new();
    for item in file_lines.into_iter() {
        lines.push(item?);
    }

    let stacks_instructions = lines.split(|x| x.is_empty()).collect::<Vec<&[String]>>();
    let stacks = stacks_instructions[0];
    let instructions = stacks_instructions[1];

    let stack_list = stacks.iter().rev().map(|x| x.split(' ').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let stacks_total = stack_list[0].iter().filter(|x| !x.is_empty()).collect::<Vec<&&str>>();
    println!("total stacks: {:?}", stacks_total);

    let mut stacks: HashMap<String, Vec<String>> = HashMap::new();
    for item in stacks_total{
        stacks.insert(item.to_string(), Vec::new());
    }

    for stack_row in &stack_list[1..] {
        let mut curr_stack = 1;
        let mut curr_loc = 0;
        while curr_loc < stack_row.len() {
            let stack_entry = stack_row.get(curr_loc).expect("failed to get stack entry");
            if !stack_entry.is_empty() {
                stacks.get_mut(&curr_stack.to_string()).expect("failed to get stack").push(stack_entry.to_string());
            } else {
                curr_loc += 3;
            }
            curr_loc += 1;
            curr_stack += 1;
        }
    }

    println!("{:?}", stacks);

    for instruction in instructions {
        let mut move_f = false;
        let mut from = false;
        let mut to = false;

        let mut inst_num = 0;
        let mut from_num = 0;
        let mut to_num = 0;
        for item in instruction.split(' ') {
            match item {
                "move" => move_f = true,
                "from" => from = true,
                "to" => to = true,
                other if move_f && !from => {
                    inst_num = other.parse().expect("failed to parse inst num");
                },
                other if from  && !to => {
                    from_num = other.parse().expect("failed to parse from num");
                },
                other if to => {
                    to_num = other.parse().expect("failed to parse to num");

                },
                _ => panic!("bad")
            }
        }

        let stack_len = stacks.get(&from_num.to_string()).expect("failed to get stack").len();
        
        let stack_vals = {
            let stack = stacks.get_mut(&from_num.to_string()).expect("failed to get stack");
            let expand = stack[stack_len-inst_num..].to_vec();
            for i in (stack_len-inst_num..stack_len).rev() {
                stack.remove(i);
            }
            expand
        };
        stacks.get_mut(&to_num.to_string()).expect("failed to get stack").extend_from_slice(&stack_vals);
    }

    println!("{:?}", stacks);

    Ok(())
}
