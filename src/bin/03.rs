use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_PART_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_PART_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

const MUL_REGEX: &str = r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)";
const DONT_REGEX: &str = r"don't\(\)";
const DO_REGEX: &str = r"do\(\)";

fn main() -> Result<()> {
    start_day(DAY);

    fn multiply_numbers(capture: Captures) -> usize {
        // let (_, [first, second]) = capture.extract(); /* Can't be used for Part 2, due to multiple grouping using | (pipe) */
        let first = &capture["first"];
        let second = &capture["second"];

        first.parse::<usize>().unwrap() * second.parse::<usize>().unwrap()
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let regex = Regex::new(MUL_REGEX).unwrap();

        let mut total = 0;
        reader.lines().for_each(|line| {
            let line = line.unwrap();
            let sub_total = regex
                .captures_iter(&line)
                .map(|c| multiply_numbers(c))
                .sum::<usize>();
            total += sub_total;
        });
        Ok(total)
    }

    assert_eq!(161, part1(BufReader::new(TEST_PART_1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let filters = format!("{MUL_REGEX}|{DONT_REGEX}|{DO_REGEX}");
        let regex = Regex::new(filters.as_str()).unwrap();

        let mut enabled = true;
        let mut total = 0;
        reader.lines().for_each(|line| {
            let line = line.unwrap();
            regex.captures_iter(&line).for_each(|c| {
                let match_string = c.get(0).unwrap().as_str();
                match match_string {
                    "don't()" => enabled = false,
                    "do()" => enabled = true,
                    _ => {
                        if enabled {
                            total += multiply_numbers(c)
                        }
                    }
                }
            });
        });
        Ok(total)
    }

    assert_eq!(48, part2(BufReader::new(TEST_PART_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
