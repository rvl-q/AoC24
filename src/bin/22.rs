use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "22"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1
10
100
2024
"; // TODOne: Add the test input
const TEST2: &str = "\
1
2
3
2024
"; // DIFFERENT secrets!
const N: u64 = 16777216;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        // TODOne: Solve Part 1 of the puzzle
        let mut nums: Vec<u64> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let s: u64 = line.parse()?;
            nums.push(s);
        }
        fn step(secret: &u64) -> u64 {
            let mut res = secret.clone();
            for _i in 0..2000 {
                res = res << 6 ^ res;
                res = res % N;
                res = res >> 5 ^ res;
                res = res % N;
                res = res << 11 ^ res;
                res = res % N;
            }
            res
        }
        let mut bids2k = Vec::new();
        for n in &nums {
            bids2k.push(step(n));
        }
        let answer = bids2k.iter().sum();
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(37327623, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut nums: Vec<u64> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let s: u64 = line.parse()?;
            nums.push(s);
        }
        fn step(secret: &u64) -> Vec<(i8, i8)> {
            let mut result = Vec::new();
            let mut res = secret.clone();
            let mut prev = (res % 10) as i8;
            result.push((prev, 99i8));
            for _i in 0..2000 {
                res = res << 6 ^ res;
                res = res % N;
                res = res >> 5 ^ res;
                res = res % N;
                res = res << 11 ^ res;
                res = res % N;
                let now = (res % 10) as i8;
                result.push((now, now - prev));
                prev = now
            }
            result
        }

        let mut big_hash = HashMap::new();
        for n in 0..nums.len() {
            let monkey = step(&nums[n]);
            let mut rset = HashSet::new();
            for i in 4..=2000 {
                if rset.contains(&(monkey[i - 3].1, monkey[i - 2].1, monkey[i - 1].1, monkey[i].1)) {
                    continue;
                }
                rset.insert((monkey[i - 3].1, monkey[i - 2].1, monkey[i - 1].1, monkey[i].1));
                if monkey[i].0 == 0 {
                    continue;
                }
                let mut tmp = monkey[i].0 as u64;
                if big_hash.contains_key(&(monkey[i - 3].1, monkey[i - 2].1, monkey[i - 1].1, monkey[i].1)) {
                    let tmptmp = big_hash.get(&(monkey[i - 3].1, monkey[i - 2].1, monkey[i - 1].1, monkey[i].1)).unwrap();
                    tmp += tmptmp;
                }
                big_hash.insert((monkey[i - 3].1, monkey[i - 2].1, monkey[i - 1].1, monkey[i].1), tmp);
            }
        }
        // println!("{}", big_hash.len());
        let answer = *big_hash.values().max().unwrap();
        println!("{answer}");
        Ok(answer)
    }

    assert_eq!(23, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
// === Part 1 ===
// src\bin\22.rs:64 took 15.9653ms.
// Result = 19822877190
//
// === Part 2 ===
// 23
// 2277
// src\bin\22.rs:126 took 823.5213ms.
// Result = 2277
//