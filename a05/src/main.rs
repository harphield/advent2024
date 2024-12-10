use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut mode = 1;

    let mut rules = vec![];
    let mut updates = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if line.eq("") {
                    mode = 2;
                    continue;
                }

                if mode == 1 {
                    let split = line.split('|').collect::<Vec<&str>>();

                    let num1 = split[0].parse::<u32>().unwrap();
                    let num2 = split[1].parse::<u32>().unwrap();

                    rules.push((num1, num2));
                } else {
                    let split = line
                        .split(',')
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    updates.push(split);
                }
            }
            Err(_) => break,
        }
    }

    let mut result = 0;
    let mut correct = true;

    let mut incorrect = vec![];

    updates.iter().for_each(|update| {
        correct = true;

        let mut positions = HashMap::new();
        update.iter().enumerate().for_each(|(pos, number)| {
            positions.insert(*number, pos);
        });

        for r in rules.iter() {
            let num1_pos = positions.get(&r.0);
            let num2_pos = positions.get(&r.1);

            if num1_pos.is_some() && num2_pos.is_some() && num1_pos.unwrap() > num2_pos.unwrap() {
                correct = false;
                incorrect.push(update.clone());
                break;
            }
        }

        if correct {
            result += update[(update.len() as f32 / 2f32).floor() as usize];
        }
    });

    println!("Part 1: {}", result);

    // sorting incorrect ones
    result = 0;

    incorrect.iter_mut().for_each(|update| {
        update.sort_by(|a, b| {
            match rules.iter().find(|(x, y)| {
                return x == a && y == b || x == b && y == a;
            }) {
                None => {}
                Some(rule) => {
                    if rule.0 == *a {
                        return Ordering::Less;
                    } else if rule.0 == *b {
                        return Ordering::Greater;
                    }
                }
            }

            Ordering::Equal
        });

        result += update[(update.len() as f32 / 2f32).floor() as usize];
    });

    println!("Part 2: {}", result);

    Ok(())
}
