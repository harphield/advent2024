use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut obstacles = vec![];
    let mut start = 0;
    let mut index = 0;
    let mut width = 0;
    let mut height = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if width == 0 {
                    width = line.len();
                }

                line.chars().for_each(|c| {
                    match c {
                        '^' => start = index,
                        '#' => obstacles.push(index),
                        _ => {}
                    }

                    index += 1;
                });

                height += 1;
            }
            Err(_) => break,
        }
    }

    let mut direction = Direction::Up;
    let mut position = start;
    let mut traveled = HashSet::new();

    loop {
        let x = position % width;
        let y = position / width;

        traveled.insert(position);

        match direction {
            Direction::Up => {
                if y == 0 {
                    break;
                }

                if obstacles.contains(&((y - 1) * width + x)) {
                    direction = Direction::Right;
                    continue;
                }

                position = (y - 1) * width + x;
            }
            Direction::Down => {
                if y == height - 1 {
                    break;
                }

                if obstacles.contains(&((y + 1) * width + x)) {
                    direction = Direction::Left;
                    continue;
                }

                position = (y + 1) * width + x;
            }
            Direction::Left => {
                if x == 0 {
                    break;
                }

                if obstacles.contains(&(y * width + x - 1)) {
                    direction = Direction::Up;
                    continue;
                }

                position = y * width + x - 1;
            }
            Direction::Right => {
                if x == width - 1 {
                    break;
                }

                if obstacles.contains(&(y * width + x + 1)) {
                    direction = Direction::Down;
                    continue;
                }

                position = y * width + x + 1;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if y * width + x == start {
                print!("^");
            } else if obstacles.contains(&(y * width + x)) {
                print!("#");
            } else if traveled.contains(&(y * width + x)) {
                print!("X");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();

    println!("Part 1: {}", traveled.len());

    Ok(())
}
