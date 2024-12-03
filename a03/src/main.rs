use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let regex = Regex::new("mul\\(([0-9]+),([0-9]+)\\)|do\\(\\)|don't\\(\\)").unwrap();

    let mut result = 0;
    let mut enabled = true;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let captures = regex.captures_iter(&line);
                captures.for_each(|c| {
                    let method = c.get(0).unwrap().as_str();

                    if method.eq("do()") {
                        enabled = true;
                    } else if method.eq("don't()") {
                        enabled = false;
                    } else if enabled {
                        result += c.get(1).unwrap().as_str().parse::<i32>().unwrap()
                            * c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    }
                })
            }
            Err(_) => break,
        }
    }

    println!("Part 2: {}", result);

    Ok(())
}
