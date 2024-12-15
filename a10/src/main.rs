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

                line.chars().for_each(|c| {
                    grid.push(c.to_digit(10).unwrap());
                });

                height += 1;
            }
            Err(_) => {}
        }
    }

    let mut result = 0;
    let mut found = vec![];
    for i in 0..grid.len() {
        if grid[i] == 0 {
            result += find_trails(&grid, width, height, i, i, 0, &mut found);
        }
    }

    println!("Part 1: {}", result);

    Ok(())
}

fn find_trails(grid: &Vec<u32>, width: usize, height: usize, start_index: usize, start: usize, next_value: u32, found: &mut Vec<(usize, usize)>) -> usize {
    if grid[start] == 9 {
        return if !found.contains(&(start_index, start)) {
            // for part 2, comment this out
            found.push((start_index, start));
            1
        } else {
            0
        }
    }

    let mut result = 0;

    let x = start % width;
    let y = start / width;

    // look left
    if x > 0 && grid[start - 1] == next_value + 1 {
        result += find_trails(grid, width, height, start_index, start - 1, next_value + 1, found);
    }
    // look right
    if x < width - 1 && grid[start + 1] == next_value + 1 {
        result += find_trails(grid, width, height, start_index, start + 1, next_value + 1, found);
    }
    // look up
    if y > 0 && grid[start - width] == next_value + 1 {
        result += find_trails(grid, width, height, start_index, start - width, next_value + 1, found);
    }
    // look down
    if y < height - 1 && grid[start + width] == next_value + 1 {
        result += find_trails(grid, width, height, start_index, start + width, next_value + 1, found);
    }

    result
}
