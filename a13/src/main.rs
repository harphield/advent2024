use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let button_regex = Regex::new("Button (.+): X\\+([0-9]+), Y\\+([0-9]+)").unwrap();
    let prize_regex = Regex::new("Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut a_x = 0f32;
    let mut a_y = 0f32;
    let mut b_x = 0f32;
    let mut b_y = 0f32;

    let mut prize_x = 0f32;
    let mut prize_y = 0f32;

    let mut found = vec![];

    let mut min_tokens = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if button_regex.is_match(&line) {
                    found = vec![];

                    let captures = button_regex.captures(&line).unwrap();
                    let letter = captures.get(1).unwrap().as_str();

                    match letter {
                        "A" => {
                            a_x = captures.get(2).unwrap().as_str().parse::<f32>().unwrap();
                            a_y = captures.get(3).unwrap().as_str().parse::<f32>().unwrap();
                        },
                        "B" => {
                            b_x = captures.get(2).unwrap().as_str().parse::<f32>().unwrap();
                            b_y = captures.get(3).unwrap().as_str().parse::<f32>().unwrap();
                        },
                        _ => {
                            panic!("oh no");
                        }
                    }
                } else if prize_regex.is_match(&line) {
                    let captures = prize_regex.captures(&line).unwrap();
                    prize_x = captures.get(1).unwrap().as_str().parse::<f32>().unwrap();
                    prize_y = captures.get(2).unwrap().as_str().parse::<f32>().unwrap();
                } else {
                    // do things
                    println!("A: {} {}, B: {} {}, Prize: {} {}", a_x, a_y, b_x, b_y, prize_x, prize_y);

                    for b in 0..=100 {
                        let ax = (prize_x - (b as f32 * b_x)) / a_x;
                        let ay = (prize_y - (b as f32 * b_y)) / a_y;

                        if ax > 0f32 && ax.floor() == ax && ax <= 100f32 && ay > 0f32 && ay.floor() == ay && ay <= 100f32 && ax == ay {
                            println!("b: {} a: {}", b, ax);
                            found.push((ax as u32, b as u32));
                        }
                    }

                    let possibilities = found.iter().map(|(a, b)| {
                        (a * 3) + b
                    }).collect::<Vec<u32>>();

                    if possibilities.len() > 0 {
                        min_tokens += possibilities.iter().min().unwrap();
                    }

                    println!();
                }
            }
            Err(_) => {}
        }
    }

    println!("Part 1: {}", min_tokens);

    Ok(())
}
