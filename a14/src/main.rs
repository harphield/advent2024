use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let regex = Regex::new("p=([0-9]+),([0-9]+) v=(.+),(.+)").unwrap();

    let width = 101;
    let height = 103;
    let seconds = 100;

    let mut quadrants = vec![0, 0, 0, 0];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let captures = regex.captures(&line).unwrap();

                let pos_x = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let pos_y = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

                let vel_x = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let vel_y = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

                let mut new_pos_x = pos_x as i32 + (seconds * vel_x) % width;
                if new_pos_x >= width {
                    new_pos_x = new_pos_x - width;
                } else if new_pos_x < 0 {
                    new_pos_x = width + new_pos_x;
                }

                let mut new_pos_y = pos_y as i32 + (seconds * vel_y) % height;
                if new_pos_y >= height {
                    new_pos_y = new_pos_y - height;
                } else if new_pos_y < 0 {
                    new_pos_y = height + new_pos_y;
                }

                if new_pos_x < width / 2 && new_pos_y < height / 2 {
                    quadrants[0] += 1;
                } else if new_pos_x > width / 2 && new_pos_y < height / 2 {
                    quadrants[1] += 1;
                } else if new_pos_x < width / 2 && new_pos_y > height / 2 {
                    quadrants[2] += 1;
                } else if new_pos_x > width / 2 && new_pos_y > height / 2 {
                    quadrants[3] += 1;
                }
            }
            Err(_) => {}
        }
    }

    println!("Quadrants: {:?}", quadrants);
    println!(
        "Part 1: {}",
        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    );

    Ok(())
}
