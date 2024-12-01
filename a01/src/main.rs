use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut list_1 = vec![];
    let mut list_2 = vec![];

    let regex = Regex::new("([0-9]+)\\s+([0-9]+)").unwrap();

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let captures = regex.captures(&line).unwrap();
                let value_1 = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                let value_2 = captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();

                list_1.push(value_1);
                list_2.push(value_2);
            }
            Err(_) => break,
        }
    }

    list_1.sort();
    list_2.sort();

    let len = list_1.len();

    let mut result = 0;
    let mut sim_score = 0;
    for i in 0..len {
        let num = list_1.get(i).unwrap();
        let diff = num - list_2.get(i).unwrap();
        result += diff.abs();

        // search how many times it's in list_2
        let cnt = list_2.iter().filter(|value| **value == *num).count();
        sim_score += cnt as i32 * num;
    }

    println!("Part 1: {}", result);
    println!("Part 2: {}", sim_score);

    Ok(())
}
