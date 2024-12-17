use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut numbers = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                numbers = line
                    .split(' ')
                    .map(|c| c.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
            }
            Err(_) => {}
        }
    }

    println!("{:?}", numbers);

    let mut numbers_table = HashMap::new();

    for n in numbers.iter() {
        *numbers_table.entry(*n).or_insert(0u64) += 1;
    }

    for _i in 0..25 {
        let mut new_numbers = vec![];

        numbers.iter().for_each(|n| {
            if n == &0 {
                new_numbers.push(1);
            } else {
                let digits = n.ilog10() as u64 + 1u64;
                if digits % 2 == 0 {
                    // split into 2
                    let number1 = n / (10u64.pow(digits as u32 / 2));
                    let number2 = n % (10u64.pow(digits as u32 / 2));

                    new_numbers.push(number1);
                    new_numbers.push(number2);
                } else {
                    new_numbers.push(n * 2024);
                }
            }
        });

        numbers = new_numbers;
    }

    println!("Part 1: {}", numbers.len());

    // part 2 is 75, new algo!
    for _i in 0..75 {
        let mut new_n_t = HashMap::new();
        numbers_table.iter().for_each(|(n, count)| {
            if n == &0 {
                *new_n_t.entry(1).or_insert(0u64) += count;
            } else {
                let digits = n.ilog10() as u64 + 1u64;

                if digits % 2 == 0 {
                    // split into 2
                    let number1 = n / (10u64.pow(digits as u32 / 2));
                    let number2 = n % (10u64.pow(digits as u32 / 2));

                    *new_n_t.entry(number1).or_insert(0u64) += count;
                    *new_n_t.entry(number2).or_insert(0u64) += count;
                } else {
                    *new_n_t.entry(n * 2024).or_insert(0u64) += count;
                }
            }
        });

        numbers_table = new_n_t;
    }

    let mut cnt = 0;
    numbers_table.iter().for_each(|(_n, count)| {
        cnt += count;
    });

    println!("Part 2: {}", cnt);

    Ok(())
}
