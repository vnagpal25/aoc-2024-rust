use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_input<R: BufRead>(reader: R) -> Result<(Vec<usize>, Vec<usize>)> {
        let rows: Vec<(usize, usize)> = reader.lines() // read the file
            .map(|line| {
                // Convert each line Result into a result of split parts
                line.map(|l| {
                    // Split the line into two parts and convert each to an numeric type
                    let parts: Vec<usize> = l.split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();

                    // Ensure we have exactly two parts
                    (parts[0].clone(), parts[1].clone())
                })
            })
            .collect::<Result<Vec<(usize, usize)>, _>>()?;

        // Separate into two vectors of 'a's and 'b's
        let ( a_values, b_values): (Vec<usize>, Vec<usize>) = rows.into_iter()
            .unzip();

        Ok((a_values, b_values))
    }
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle

        // Separate into two vectors of 'a's and 'b's
        let (mut a_values, mut b_values): (Vec<usize>, Vec<usize>) = parse_input(reader)?;

        // sort a and b values
        a_values.sort();
        b_values.sort();

        // sum the absolute differences of corresponding values
        let answer = a_values.iter().zip(b_values.iter()).map(|(a, b)| a.abs_diff(*b)).sum::<usize>();
        // println!("Answer = {:?}", answer);
        //
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // let mut answer = 0;
        // Split each line into parts, convert to a vector of tuples, then separate into two vectors
        let (left, right): (Vec<usize>, Vec<usize>) = parse_input(reader)?;

        // count how many times each element of a appears in b
        let counts: Vec<usize> = left.iter().map(|&el| right.iter().filter(|&x| *x == el).count()).collect();
        let answer = left.iter().zip(counts.iter()).map(|(a, b)| a*b).sum::<usize>();
        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
