use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "11"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        // TODOne: Solve Part 1 of the puzzle
        let mut stones: Vec<u64> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            // println!("{}", line.len());
            let tmp = line.split(' ');
            // println!("{tmp:?}");
            for s in tmp {
                let s: u64 = s.parse()?;
                // println!("{s:?}");
                stones.push(s);
            }
        }
        // println!("{stones:?}");
        let mut stonex: HashMap<u64, u64> = HashMap::new();
        for stone in &stones {
            let n = stonex.get(stone).unwrap_or(&0u64);
            stonex.insert(*stone, n + 1);
        }
        // println!("{stonex:?}");
        fn mod_stone(stone: u64) -> Vec<u64> {
            let mut res = Vec::new();
            if stone == 0 {
                res.push(1);
            } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
                let n = (stone.checked_ilog10().unwrap_or(0) + 1) / 2;
                // println!("n: {n}");
                let exn = 10u64.pow(n);
                let hh = stone / exn;
                let lh = stone % exn;
                res.push(hh);
                res.push(lh);
            } else {
                let r = 2024 * stone;
                res.push(r);
            }
            res
        }
        let mut stoney: HashMap<u64, u64> = HashMap::new();
        for (k, v) in &stonex {
            let tmp = mod_stone(*k);
            // println!("tmp: {tmp:?}")
            for stone in tmp {
                let n = stoney.get(&stone).unwrap_or(&0u64);
                stoney.insert(stone, n + 1u64 * *v);
            }
        }
        // 25-1 / 2 = 12
        // 75-1 / 2 = 37
        for _i in 0..12 {
            stonex.clear();
            for (k, v) in &stoney {
                let tmp = mod_stone(*k);
                // println!("tmp: {tmp:?}")
                for stone in tmp {
                    let n = stonex.get(&stone).unwrap_or(&0u64);
                    stonex.insert(stone, n + 1u64 * *v);
                }
            }
            stoney.clear();
            for (k, v) in &stonex {
                let tmp = mod_stone(*k);
                // println!("tmp: {tmp:?}")
                for stone in tmp {
                    let n = stoney.get(&stone).unwrap_or(&0u64);
                    stoney.insert(stone, n + 1u64 * *v);
                }
            }
        }
        // println!("{stonex:?}");
        // println!("{stoney:?}");
        println!("{}", stoney.len());
        let mut answer = 0;
        // let mut keys = Vec::new();
        for (_k, v) in stoney {
            // print!("{_k} ");
            // keys.push(_k);
            answer += v;
        }
        // keys.sort();
        // println!("{keys:?}");
        println!("{answer}");
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    // p1: 55312, p2: 65601038650482
    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut stones: Vec<u64> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            // println!("{}", line.len());
            let tmp = line.split(' ');
            // println!("{tmp:?}");
            for s in tmp {
                let s: u64 = s.parse()?;
                // println!("{s:?}");
                stones.push(s);
            }
        }
        // println!("{stones:?}");
        let mut stonex: HashMap<u64, u64> = HashMap::new();
        for stone in &stones {
            let n = stonex.get(stone).unwrap_or(&0u64);
            stonex.insert(*stone, n + 1);
        }
        // println!("{stonex:?}");
        fn mod_stone(stone: u64) -> Vec<u64> {
            let mut res = Vec::new();
            if stone == 0 {
                res.push(1);
            } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
                let n = (stone.checked_ilog10().unwrap_or(0) + 1) / 2;
                // println!("n: {n}");
                let exn = 10u64.pow(n);
                let hh = stone / exn;
                let lh = stone % exn;
                res.push(hh);
                res.push(lh);
            } else {
                let r = 2024 * stone;
                res.push(r);
            }
            res
        }
        let mut stoney: HashMap<u64, u64> = HashMap::new();
        for (k, v) in &stonex {
            let tmp = mod_stone(*k);
            // println!("tmp: {tmp:?}")
            for stone in tmp {
                let n = stoney.get(&stone).unwrap_or(&0u64);
                stoney.insert(stone, n + 1u64 * *v);
            }
        }
        // 25-1 / 2 = 12
        // 75-1 / 2 = 37
        for _i in 0..37 {
            stonex.clear();
            for (k, v) in &stoney {
                let tmp = mod_stone(*k);
                // println!("tmp: {tmp:?}")
                for stone in tmp {
                    let n = stonex.get(&stone).unwrap_or(&0u64);
                    stonex.insert(stone, n + 1u64 * *v);
                }
            }
            stoney.clear();
            for (k, v) in &stonex {
                let tmp = mod_stone(*k);
                // println!("tmp: {tmp:?}")
                for stone in tmp {
                    let n = stoney.get(&stone).unwrap_or(&0u64);
                    stoney.insert(stone, n + 1u64 * *v);
                }
            }
        }
        // println!("{stonex:?}");
        // println!("{stoney:?}");
        println!("{}", stoney.len());
        let mut answer = 0;
        // let mut keys = Vec::new();
        for (_k, v) in stoney {
            // print!("{_k} ");
            // keys.push(_k);
            answer += v;
        }
        // keys.sort();
        // println!("{keys:?}");
        println!("{answer}");
        Ok(answer)
    }

    assert_eq!(65601038650482, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
