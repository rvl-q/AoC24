use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "00"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn l2num(line: String) -> usize {
        let mut ans = 0;
        for ch in line.chars() {
            println!("ch: {}, num? {}", ch, ch.is_ascii_digit());
        }
        ans
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        for line in reader.lines(){
            match line {
                std::result::Result::Ok(l) => {
                    println!("yes, {}", l);
                    answer += l2num(l.clone());
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        //println!("{}", lines);
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(142, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
