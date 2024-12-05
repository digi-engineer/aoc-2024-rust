use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

const MIN_ABS_DIFF: usize = 1;
const MAX_ABS_DIFF: usize = 3;

fn main() -> Result<()> {
    start_day(DAY);

    fn convert_reports_to_levels_list<R: BufRead>(reader: R) -> Vec<Vec<usize>> {
        reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                line.split_whitespace()
                    .map(|l| l.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn is_increasing_properly(levels: &Vec<usize>) -> bool {
        for i in 0..levels.len() - 1 {
            let diff = levels[i + 1].abs_diff(levels[i]);
            if levels[i] > levels[i + 1] || diff < MIN_ABS_DIFF || diff > MAX_ABS_DIFF {
                return false;
            }
        }
        true
    }

    fn is_decreasing_properly(levels: &Vec<usize>) -> bool {
        for i in 0..levels.len() - 1 {
            let diff = levels[i + 1].abs_diff(levels[i]);
            if levels[i + 1] > levels[i] || diff < MIN_ABS_DIFF || diff > MAX_ABS_DIFF {
                return false;
            }
        }
        true
    }

    fn is_safe_report(levels: &Vec<usize>) -> bool {
        is_increasing_properly(levels) || is_decreasing_properly(levels)
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let levels_list = convert_reports_to_levels_list(reader);
        let result = levels_list
            .iter()
            .map(| levels | if is_safe_report(levels) { 1 } else { 0 })
            .sum();
        Ok(result)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let levels_list = convert_reports_to_levels_list(reader);
        let result = levels_list
            .iter()
            .map(|levels| {
                if is_safe_report(levels) {
                    return 1;
                }
                for i in 0..levels.len() {
                    let mut dampened_levels = levels.clone();
                    dampened_levels.remove(i);
                    if is_safe_report(&dampened_levels) {
                        return 1
                    }
                }
                return 0;
            })
            .sum();
        Ok(result)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
