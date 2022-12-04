use std::{io::{BufReader, BufRead}, fs::File, collections::HashSet};

fn set_generation(in_file: BufReader<File>) -> Result<Vec<(HashSet<i32>, HashSet<i32>)>, std::io::Error> {
    let file_lines = in_file.lines().collect::<Vec<Result<String, std::io::Error>>>();

    let mut lines = Vec::new();
    for item in file_lines.into_iter() {
        lines.push(item?);
    }

    let ranges = lines
        .into_iter()
        .map(|x| { 
            let iter = x.split(",").collect::<Vec<&str>>();
            (iter[0].to_string(), iter[1].to_string())
        })
        .collect::<Vec<(String, String)>>();

    Ok(ranges
        .into_iter()
        .map(|(left, right)| {
            let l_split = left.split("-").collect::<Vec<&str>>();
            let l_left = l_split[0].parse::<i32>().expect("failed to parse l_left");
            let l_right = l_split[1].parse::<i32>().expect("failed to parse l_right");

            let r_split = right.split("-").collect::<Vec<&str>>();
            let r_left = r_split[0].parse::<i32>().expect("failed to parse r_left");
            let r_right = r_split[1].parse::<i32>().expect("failed to parse r_right");

            let mut left_set = HashSet::new();
            for i in l_left..=l_right {
                left_set.insert(i);
            }

            
            let mut right_set = HashSet::new();
            for i in r_left..=r_right {
                right_set.insert(i);
            }

            (left_set, right_set)
        }).collect::<Vec<(HashSet<i32>, HashSet<i32>)>>())
}

pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let sets = set_generation(in_file)?;

    let subsets = sets
        .into_iter()
        .map(|x| x.0.is_subset(&x.1) || x.1.is_subset(&x.0))
        .filter(|x| *x)
        .count();
        
    println!("Subset Count: {}", subsets);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let sets = set_generation(in_file)?;

    let intersections = sets
        .into_iter()
        .map(|x| x.0.intersection(&x.1).count() > 0)
        .filter(|x| *x)
        .count();

    println!("Intersection Count: {}", intersections);

    Ok(())
}