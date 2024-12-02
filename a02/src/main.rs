use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut safe_count = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let numbers = line
                    .split(" ")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                let safe = check_safety(&numbers, true);

                if safe {
                    safe_count += 1;
                }
            }
            Err(_) => break,
        }
    }

    println!("Part 2: {}", safe_count);

    Ok(())
}

fn check_safety(numbers: &[u32], try_to_fix: bool) -> bool {
    let mut safe = true;

    numbers.iter().enumerate().for_each(|(i, x)| {
        if i > 0 {
            if (numbers[i - 1] == *x)
                || (numbers[i - 1] > *x && !(1..=3u32).contains(&(numbers[i - 1] - *x))
                    || (numbers[i - 1] < *x && !(1..=3u32).contains(&(*x - numbers[i - 1]))))
            {
                safe = false;
            }

            if i < numbers.len() - 1
                && ((numbers[i - 1] > *x && numbers[i + 1] > *x)
                    || (numbers[i - 1] < *x && numbers[i + 1] < *x))
            {
                safe = false;
            }
        }
    });

    if !safe && try_to_fix {
        // try if removing one value fixes it
        for i in 0..numbers.len() {
            let mut new_numbers = numbers.to_vec();
            new_numbers.remove(i);

            if check_safety(&new_numbers, false) {
                safe = true;
                break;
            }
        }
    }

    safe
}
