use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut sum = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let split = line.split(' ').collect::<Vec<&str>>();

                let result = split
                    .get(0)
                    .unwrap()
                    .replace(":", "")
                    .parse::<u64>()
                    .unwrap();
                let operands = split[1..]
                    .iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                let mut test_result = try_operation(result, 0, &operands, 0, Operator::Times);
                if test_result == 0 {
                    test_result = try_operation(result, 0, &operands, 0, Operator::Plus);
                }
                if test_result == 0 {
                    test_result = try_operation(result, 0, &operands, 0, Operator::Concat);
                }
                println!("{} = {:?} = {}", result, operands, test_result);

                if test_result != 0 {
                    sum += test_result;
                }
            }
            Err(_) => {}
        }
    }

    println!("Part 1: {}", sum);

    Ok(())
}

enum Operator {
    Plus,
    Times,
    Concat,
}

fn try_operation(
    final_result: u64,
    current_result: u64,
    operands: &Vec<u64>,
    current_operand: usize,
    operator: Operator,
) -> u64 {
    let mut result: u64 = 0;

    if current_result == 0 {
        result = try_operation(
            final_result,
            operands[current_operand],
            &operands,
            current_operand + 1,
            operator,
        );
    } else if current_operand == operands.len() || current_result > final_result {
        result = current_result;
    } else {
        let new_current_result = match operator {
            Operator::Plus => current_result + operands[current_operand],
            Operator::Times => current_result * operands[current_operand],
            Operator::Concat => format!("{}{}", current_result, operands[current_operand])
                .parse::<u64>()
                .unwrap(),
        };

        let result_times = try_operation(
            final_result,
            new_current_result,
            &operands,
            current_operand + 1,
            Operator::Times,
        );

        if result_times == final_result {
            result = result_times;
        } else {
            let result_plus = try_operation(
                final_result,
                new_current_result,
                &operands,
                current_operand + 1,
                Operator::Plus,
            );
            if result_plus == final_result {
                result = result_plus;
            } else {
                let result_concat = try_operation(
                    final_result,
                    new_current_result,
                    &operands,
                    current_operand + 1,
                    Operator::Concat,
                );
                if result_concat == final_result {
                    result = result_concat;
                }
            }
        }
    }

    result
}
