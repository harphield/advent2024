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
        }
    }

    println!("{:?}", nodes);

    Ok(())
}

/// manhattan distance
fn get_distance(pos1: usize, pos2: usize, width: usize) -> usize {
    let x1 = pos1 % width;
    let y1 = pos1 / width;
    let x2 = pos2 % width;
    let y2 = pos2 / width;

    //let 2 * distance = ((qx * x2 as i32 - x1 as i32).abs() + (qy * y2 as i32 - y1 as i32).abs()) as usize;


    0
}

fn get_position_in_distance(pos1: usize, pos2: usize, width: usize) -> usize {
    let distance = get_distance(pos1, pos2, width);

    0
}