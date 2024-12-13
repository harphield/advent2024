use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut nodes = vec![];

    let mut width = 0;
    let mut height = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if width == 0 {
                    width = line.len();
                }

                line.chars().enumerate().for_each(|(i, c)| {
                    if !c.eq(&'.') {
                        nodes.push((c, height * width + i));
                    }
                });

                height += 1;
            }
            Err(_) => {}
        };
    }

    // println!("{:?}", nodes);

    let mut antinodes = HashSet::new();
    let mut antinodes_part_2 = HashSet::new();

    for (i, n) in nodes.iter().enumerate() {
        // find pairs for n
        for n2 in nodes[i + 1..].iter() {
            if n2.0.eq(&n.0) {
                match get_pos_in_double_distance(n.1, n2.1, width, height) {
                    None => {}
                    Some(pos) => {
                        antinodes.insert(pos);
                    }
                };

                match get_pos_in_double_distance(n2.1, n.1, width, height) {
                    None => {}
                    Some(pos) => {
                        antinodes.insert(pos);
                    }
                };

                // part 2
                for p in get_pos_in_distances(n.1, n2.1, width, height).iter() {
                    antinodes_part_2.insert(*p);
                }

                for p in get_pos_in_distances(n2.1, n.1, width, height).iter() {
                    antinodes_part_2.insert(*p);
                }
            }
        }
    }

    println!("Part 1: {}", antinodes.len());

    println!("Part 2: {}", antinodes_part_2.len());

    Ok(())
}

/// manhattan distance
fn get_pos_in_double_distance(
    pos1: usize,
    pos2: usize,
    width: usize,
    height: usize,
) -> Option<usize> {
    let x1 = pos1 % width;
    let y1 = pos1 / width;
    let x2 = pos2 % width;
    let y2 = pos2 / width;

    let x3 = 2i32 * x2 as i32 - x1 as i32;
    let y3 = 2i32 * y2 as i32 - y1 as i32;

    if x3 >= 0 && y3 >= 0 && x3 < width as i32 && y3 < height as i32 {
        //println!("found: {} {}", x3, y3);
        return Some(y3 as usize * width + x3 as usize);
    }

    None
}

fn get_pos_in_distances(pos1: usize, pos2: usize, width: usize, height: usize) -> Vec<usize> {
    let mut positions = vec![];

    let mut x1 = pos1 % width;
    let mut y1 = pos1 / width;
    let mut x2 = pos2 % width;
    let mut y2 = pos2 / width;

    // first position is the second node
    positions.push(y2 * width + x2);

    loop {
        let x3 = 2i32 * x2 as i32 - x1 as i32;
        let y3 = 2i32 * y2 as i32 - y1 as i32;

        if x3 >= 0 && y3 >= 0 && x3 < width as i32 && y3 < height as i32 {
            //println!("found: {} {}", x3, y3);
            positions.push(y3 as usize * width + x3 as usize);
            x1 = x2;
            y1 = y2;
            x2 = x3 as usize;
            y2 = y3 as usize;
        } else {
            break;
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_pos_in_double_distance() {
        let width = 10usize;
        let pos1 = 34usize;
        let pos2 = 55usize;

        let x1 = pos1 % width;
        let y1 = pos1 / width;
        let x2 = pos2 % width;
        let y2 = pos2 / width;

        let distance = ((x2 as i32 - x1 as i32).abs() + (y2 as i32 - y1 as i32).abs()) as usize;

        let x3 = 2i32 * x2 as i32 - x1 as i32;
        let y3 = 2i32 * y2 as i32 - y1 as i32;

        println!("x3 = {}, y3 = {}", x3, y3);

        // distance from x3y3 to x2y2 should be the same as x2y2 to x1y1
        let distance2 = ((x3 - x2 as i32).abs() + (y3 - y2 as i32).abs()) as usize;

        assert_eq!(distance, distance2);

        let x3 = 2i32 * x1 as i32 - x2 as i32;
        let y3 = 2i32 * y1 as i32 - y2 as i32;

        println!("x3 = {}, y3 = {}", x3, y3);

        // distance from x3y3 to x2y2 should be the same as x2y2 to x1y1
        let distance2 = ((x3 - x1 as i32).abs() + (y3 - y1 as i32).abs()) as usize;
    }
}
