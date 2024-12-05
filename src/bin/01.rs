use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    fn get_number_pairs<R: BufRead>(reader: R) -> (Vec<usize>, Vec<usize>) {
        let regex = Regex::new(r"\s+").unwrap();
        reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let numbers = regex.split(&line).collect::<Vec<&str>>();
                let first_number = numbers[0].parse::<usize>().unwrap();
                let second_number = numbers[1].parse::<usize>().unwrap();
                (first_number, second_number)
            })
            .unzip()
    }

    #[allow(dead_code)]
    fn get_frequencies_v1(numbers: Vec<usize>) -> HashMap<usize, usize> {
        let mut count_map = HashMap::new();

        for num in numbers {
            *count_map.entry(num).or_insert(0) += 1;
        }

        count_map
    }

    fn get_frequencies(numbers: Vec<usize>) -> HashMap<usize, usize> {
        numbers
            .into_iter()
            .into_group_map_by(|&r| r)
            .into_iter()
            .map(|(k, v)| (k, v.len()))
            .collect::<HashMap<usize, usize>>()
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (left, right) = get_number_pairs(reader);

        let result = left
            .iter()
            .sorted()
            .zip(right.iter().sorted())
            .map(|(l, r)| l.abs_diff(*r))
            .sum();

        Ok(result)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left, right) = get_number_pairs(reader);

        let frequencies = get_frequencies(right);

        let similarity_score = left.iter().fold(0, |sum, value| {
            sum + value * frequencies.get(value).cloned().unwrap_or(0)
        });
        Ok(similarity_score)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
