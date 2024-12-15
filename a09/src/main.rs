use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut is_file = true;

    let mut blocks = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let mut index = 0;
                line.chars().for_each(|c| {
                    let number = c.to_digit(10).unwrap();

                    if is_file {
                        blocks.push((Some(index), number));
                        index += 1;
                    } else {
                        blocks.push((None, number));
                    }

                    is_file = !is_file;
                });
            }
            Err(_) => {}
        }
    }

    let mut i = 0;
    loop {
        if i >= blocks.len() {
            break;
        }

        match blocks[i].0 {
            None => {
                let mut kill = true;
                // empty space, we need to fill it
                for j in (i..blocks.len()).rev() {
                    match blocks[j].0 {
                        None => {}
                        Some(index) => {
                            if blocks[i].1 > blocks[j].1 {
                                // we have more space
                                let amount = blocks[i].1 - blocks[j].1;
                                let new_space = (None, amount);
                                let value = blocks[j].1;
                                blocks.remove(i);
                                blocks.insert(i, new_space);
                                blocks.remove(j);
                                blocks.insert(i, (Some(index), value));
                            } else if blocks[i].1 == blocks[j].1 {
                                // we have exactly enough space
                                let value = blocks[j].1;
                                blocks.remove(i);
                                blocks.insert(i, (Some(index), value));
                                blocks.remove(j);
                            } else {
                                // we don't have enough space
                                let amount = blocks[j].1 - blocks[i].1;
                                let value = blocks[i].1;
                                blocks.remove(i);
                                blocks.insert(i, (Some(index), value));
                                blocks[j].1 = amount;
                            }

                            //println!("{:?}", blocks);
                            kill = false;
                            break;
                        }
                    }
                }

                if kill {
                    break;
                }
            }
            Some(_) => {
                i += 1;
            }
        }
    }

    let mut checksum = 0u64;
    let mut pos = 0;
    blocks.iter().for_each(|(index, number)| match index {
        None => {}
        Some(i) => {
            for n in 0..*number {
                checksum += pos as u64 * i;

                pos += 1;
            }
        }
    });

    println!("Part 1: {}", checksum);

    Ok(())
}
