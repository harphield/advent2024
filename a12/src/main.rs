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
                    grid.push(c);
                });

                height += 1;
            }
            Err(_) => {}
        }
    }

    let mut regions = vec![];

    for i in 0..grid.len() {
        if regions.iter().any(|r: &Vec<usize>| r.contains(&i)) {
            continue;
        }

        let mut region = vec![];
        fill_region(&grid, width, height, i, &mut region);

        regions.push(region);
    }

    let mut total_price = 0;

    regions.iter().for_each(|region| {
        let area = region.len();

        // perimeter
        let mut perimeter = 0;
        region.iter().for_each(|index| {
            let x = index % width;
            let y = index / width;

            // top edge
            if y == 0 || grid[index - width] != grid[*index] {
                perimeter += 1;
            }
            // left edge
            if x == 0 || grid[index - 1] != grid[*index] {
                perimeter += 1;
            }
            // right edge
            if x == width - 1 || grid[index + 1] != grid[*index] {
                perimeter += 1;
            }
            // bottom edge
            if y == height - 1 || grid[index + width] != grid[*index] {
                perimeter += 1;
            }
        });

        total_price += area * perimeter;
    });

    println!("Part 1: {}", total_price);

    Ok(())
}

fn fill_region(
    grid: &Vec<char>,
    width: usize,
    height: usize,
    index: usize,
    region: &mut Vec<usize>,
) {
    if region.contains(&index) {
        return;
    }

    region.push(index);

    let x = index % width;
    let y = index / width;

    let value = grid[index];

    // look left
    if x > 0 && grid[index - 1] == value {
        fill_region(grid, width, height, index - 1, region);
    }
    // look right
    if x < width - 1 && grid[index + 1] == value {
        fill_region(grid, width, height, index + 1, region);
    }
    // look up
    if y > 0 && grid[index - width] == value {
        fill_region(grid, width, height, index - width, region);
    }
    // look down
    if y < height - 1 && grid[index + width] == value {
        fill_region(grid, width, height, index + width, region);
    }
}
