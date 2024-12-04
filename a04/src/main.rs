use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut width = 0;
    let mut height = 0;

    let mut grid = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if width == 0 {
                    width = line.len();
                }

                line.chars().for_each(|c| grid.push(c));

                height += 1;
            }
            Err(_) => break,
        }
    }

    for (i, c) in grid.iter().enumerate() {
        print!("{}", c);

        if (i + 1) % width == 0 {
            println!();
        }
    }

    let cnt = find_xmas(&grid, width, height);

    println!("Part 1: {}", cnt);

    println!("Part 2: {}", find_x(&grid, width, height));

    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn find_xmas(grid: &Vec<char>, width: usize, height: usize) -> u32 {
    let mut count = 0;

    for (i, c) in grid.iter().enumerate() {
        let x = i % width;
        let y = i / width;

        if c.eq(&'X') {
            count += find_letter('M', grid, &Direction::Up, x, y, width, height);
            count += find_letter('M', grid, &Direction::Down, x, y, width, height);
            count += find_letter('M', grid, &Direction::Right, x, y, width, height);
            count += find_letter('M', grid, &Direction::Left, x, y, width, height);
            count += find_letter('M', grid, &Direction::UpLeft, x, y, width, height);
            count += find_letter('M', grid, &Direction::UpRight, x, y, width, height);
            count += find_letter('M', grid, &Direction::DownRight, x, y, width, height);
            count += find_letter('M', grid, &Direction::DownLeft, x, y, width, height);
        }
    }

    count
}

fn find_letter(
    letter: char,
    grid: &Vec<char>,
    direction: &Direction,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> u32 {
    match direction {
        Direction::Up => {
            if y > 0 && grid[(y - 1) * width + x] == letter {
                return match next_letter(letter) {
                    None => 1,
                    Some(l) => find_letter(l, grid, &Direction::Up, x, y - 1, width, height),
                };
            }
        }
        Direction::Down => {
            if y < height - 1 && grid[(y + 1) * width + x] == letter {
                return match next_letter(letter) {
                    None => 1,
                    Some(l) => find_letter(l, grid, &Direction::Down, x, y + 1, width, height),
                };
            }
        }
        Direction::Left => {
            if x % width > 0 && grid[(x - 1) + y * width] == letter {
                return match next_letter(letter) {
                    None => 1,
                    Some(l) => find_letter(l, grid, &Direction::Left, x - 1, y, width, height),
                };
            }
        }
        Direction::Right => {
            if x % width < width - 1 && grid[(x + 1) + y * width] == letter {
                return match next_letter(letter) {
                    None => 1,
                    Some(l) => find_letter(l, grid, &Direction::Right, x + 1, y, width, height),
                };
            }
        }
        Direction::UpLeft => {
            if y > 0 && x % width > 0 && grid[(x - 1) + (y - 1) * width] == letter {
                return match next_letter(letter) {
                    None => 1,
                    Some(l) => {
                        find_letter(l, grid, &Direction::UpLeft, x - 1, y - 1, width, height)
                    }
                };
            }
        }
        Direction::UpRight => {
            if y > 0 && x % width < width - 1 && grid[(x + 1) + (y - 1) * width] == letter {
                return match next_letter(letter) {
                    None => 1,
                    Some(l) => {
                        find_letter(l, grid, &Direction::UpRight, x + 1, y - 1, width, height)
                    }
                };
            }
        }
        Direction::DownLeft => {
            if y < height - 1 && x % width > 0 && grid[(x - 1) + (y + 1) * width] == letter {
                return match next_letter(letter) {
                    None => 1,
                    Some(l) => {
                        find_letter(l, grid, &Direction::DownLeft, x - 1, y + 1, width, height)
                    }
                };
            }
        }
        Direction::DownRight => {
            if y < height - 1 && x % width < width - 1 && grid[(x + 1) + (y + 1) * width] == letter
            {
                return match next_letter(letter) {
                    None => 1,
                    Some(l) => {
                        find_letter(l, grid, &Direction::DownRight, x + 1, y + 1, width, height)
                    }
                };
            }
        }
    }

    0
}

fn next_letter(letter: char) -> Option<char> {
    match letter {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        _ => None,
    }
}

fn find_x(grid: &Vec<char>, width: usize, height: usize) -> usize {
    let mut count = 0;

    for (i, c) in grid.iter().enumerate() {
        let x = i % width;
        let y = i / width;

        if c.eq(&'A') {
            // MMASS
            if (y > 0 && x % width > 0 && grid[(x - 1) + (y - 1) * width] == 'M')
                && (y > 0 && x % width < width - 1 && grid[(x + 1) + (y - 1) * width] == 'M')
                && (y < height - 1 && x % width > 0 && grid[(x - 1) + (y + 1) * width] == 'S')
                && (y < height - 1
                    && x % width < width - 1
                    && grid[(x + 1) + (y + 1) * width] == 'S')
            {
                count += 1;
                continue;
            }

            // SSAMM
            if (y > 0 && x % width > 0 && grid[(x - 1) + (y - 1) * width] == 'S')
                && (y > 0 && x % width < width - 1 && grid[(x + 1) + (y - 1) * width] == 'S')
                && (y < height - 1 && x % width > 0 && grid[(x - 1) + (y + 1) * width] == 'M')
                && (y < height - 1
                    && x % width < width - 1
                    && grid[(x + 1) + (y + 1) * width] == 'M')
            {
                count += 1;
                continue;
            }

            // MSAMS
            if (y > 0 && x % width > 0 && grid[(x - 1) + (y - 1) * width] == 'M')
                && (y > 0 && x % width < width - 1 && grid[(x + 1) + (y - 1) * width] == 'S')
                && (y < height - 1 && x % width > 0 && grid[(x - 1) + (y + 1) * width] == 'M')
                && (y < height - 1
                    && x % width < width - 1
                    && grid[(x + 1) + (y + 1) * width] == 'S')
            {
                count += 1;
                continue;
            }

            // SMASM
            if (y > 0 && x % width > 0 && grid[(x - 1) + (y - 1) * width] == 'S')
                && (y > 0 && x % width < width - 1 && grid[(x + 1) + (y - 1) * width] == 'M')
                && (y < height - 1 && x % width > 0 && grid[(x - 1) + (y + 1) * width] == 'S')
                && (y < height - 1
                    && x % width < width - 1
                    && grid[(x + 1) + (y + 1) * width] == 'M')
            {
                count += 1;
                continue;
            }
        }
    }

    count
}
