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

    // println!("{:?}", blocks);

    let mut i = blocks.len() - 1;
    loop {
        if i == 0 {
            break;
        }

        match blocks[i].0 {
            Some(index) => {
                // empty space, we need to fill it
                let mut found = false;

                for j in 0..i {
                    match blocks[j].0 {
                        None => {
                            if blocks[j].1 > blocks[i].1 {
                                // we have more space
                                let amount = blocks[j].1 - blocks[i].1;
                                let new_space = (None, amount);
                                let value = blocks[i].1;
                                blocks.remove(j);
                                blocks.insert(j, new_space);
                                blocks.remove(i);
                                blocks.insert(i, (None, value));
                                blocks.insert(j, (Some(index), value));
                                found = true;
                                break;
                            } else if blocks[i].1 == blocks[j].1 {
                                // we have exactly enough space
                                let value = blocks[i].1;
                                blocks.remove(j);
                                blocks.insert(j, (Some(index), value));
                                blocks.remove(i);
                                blocks.insert(i, (None, value));
                                found = true;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                if !found {
                    i -= 1;
                }
            }
            None => {
                i -= 1;
            }
        }
    }

    let mut checksum = 0u64;
    let mut pos = 0;
    blocks.iter().for_each(|(index, number)| match index {
        None => {
            pos += number;
        }
        Some(i) => {
            for _n in 0..*number {
                checksum += pos as u64 * i;

                pos += 1;
            }
        }
    });

    println!("Part 2: {}", checksum);

    Ok(())
}
