use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut is_file = true;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                line.chars().for_each(|c| {
                    let number = c.to_digit(10).unwrap();

                    if is_file {

                    }

                    is_file = !is_file;
                });
            }
            Err(_) => {}
        }
    }

    Ok(())
}
