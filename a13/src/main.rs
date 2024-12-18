use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let button_regex = Regex::new("Button (.+): X\\+([0-9]+), Y\\+([0-9]+)").unwrap();
    let prize_regex = Regex::new("Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut a_x = 0f64;
    let mut a_y = 0f64;
    let mut b_x = 0f64;
    let mut b_y = 0f64;

    let mut prize_x = 0f64;
    let mut prize_y = 0f64;

    let mut found = vec![];

    let mut min_tokens = 0;
    let mut min_tokens_2 = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if button_regex.is_match(&line) {
                    found = vec![];

                    let captures = button_regex.captures(&line).unwrap();
                    let letter = captures.get(1).unwrap().as_str();

                    match letter {
                        "A" => {
                            a_x = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
                            a_y = captures.get(3).unwrap().as_str().parse::<f64>().unwrap();
                        }
                        "B" => {
                            b_x = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
                            b_y = captures.get(3).unwrap().as_str().parse::<f64>().unwrap();
                        }
                        _ => {
                            panic!("oh no");
                        }
                    }
                } else if prize_regex.is_match(&line) {
                    let captures = prize_regex.captures(&line).unwrap();
                    prize_x = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
                    prize_y = captures.get(2).unwrap().as_str().parse::<f64>().unwrap();
                } else {
                    // do things
                    println!(
                        "A: {} {}, B: {} {}, Prize: {} {}",
                        a_x, a_y, b_x, b_y, prize_x, prize_y
                    );

                    for b in 0..=100 {
                        let ax = (prize_x - (b as f64 * b_x)) / a_x;
                        let ay = (prize_y - (b as f64 * b_y)) / a_y;

                        if ax > 0f64
                            && ax.floor() == ax
                            && ax <= 100f64
                            && ay > 0f64
                            && ay.floor() == ay
                            && ay <= 100f64
                            && ax == ay
                        {
                            println!("b: {} a: {}", b, ax);
                            found.push((ax as u32, b as u32));
                        }
                    }

                    let possibilities =
                        found.iter().map(|(a, b)| (a * 3) + b).collect::<Vec<u32>>();

                    if possibilities.len() > 0 {
                        min_tokens += possibilities.iter().min().unwrap();
                    }

                    // Part 2
                    prize_x = prize_x + 10000000000000f64;
                    prize_y = prize_y + 10000000000000f64;

                    let a_maybe = (prize_y * b_x - prize_x * b_y) / (b_x * a_y - a_x * b_y);
                    let b_maybe_x = (prize_x - a_maybe * a_x) / b_x;
                    let b_maybe_y = (prize_y - a_maybe * a_y) / b_y;

                    if b_maybe_x == b_maybe_y
                        && a_maybe.floor() == a_maybe
                        && b_maybe_x.floor() == b_maybe_x
                    {
                        println!("a maybe: {} b maybe: {}", a_maybe, b_maybe_x);
                        min_tokens_2 += a_maybe as u64 * 3 + b_maybe_x as u64;
                    }

                    println!();
                }
            }
            Err(_) => {}
        }
    }

    println!("Part 1: {}", min_tokens);
    println!("Part 2: {}", min_tokens_2);

    Ok(())
}
