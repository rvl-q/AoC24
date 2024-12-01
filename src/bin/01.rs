use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // TODOne: Solve Part 1 of the puzzle
        let mut lefts: Vec<i32> = Vec::new();
        let mut rights: Vec<i32> = Vec::new();
        for line in reader.lines() {
            match line {
                std::result::Result::Ok(l) => {
                    let parts: Vec<&str> = l.split("   ").collect();
                    //println!("yes, {}, {}, {}", l, parts[0], parts[1]);
                    //let x: usize = parts[0].parse()?;
                    //println!("{x}");
                    lefts.push(parts[0].parse()?);
                    rights.push(parts[1].parse()?);
                    //answer += l2num(l.clone());
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        //println!("{lefts:?}");
        //println!("{rights:?}");
        lefts.sort();
        rights.sort();
        //println!("{lefts:?}");
        //println!("{rights:?}");
        //let add = |a, b| a + b;
        let mut difs: Vec<i32> = Vec::new();
        lefts.iter().zip(rights.iter())
            .for_each(|(a, b)| difs.push(a - b));
        //println!("{difs:?}");
        let mut answer = 0; //reader.lines().flatten().count();
        difs.into_iter()
            .for_each(|d| answer += d.abs());
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut lefts: Vec<i32> = Vec::new();
        let mut rights: Vec<i32> = Vec::new();
        for line in reader.lines() {
            match line {
                std::result::Result::Ok(l) => {
                    let parts: Vec<&str> = l.split("   ").collect();
                    lefts.push(parts[0].parse()?);
                    rights.push(parts[1].parse()?);
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        lefts.sort();
        rights.sort();
        let mut ans = 0i32;
        // let mut _prds: Vec<i32> = Vec::new();
        lefts.iter().for_each(
            |x| {
                //print!("{x} ");
                let n = rights.iter().filter(|&n| *n == *x).count();
                //println!("{n}")
                ans += x * n as i32
            }
        );
        Ok(ans)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
