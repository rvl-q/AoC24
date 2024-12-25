use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "25"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODOne: Solve Part 1 of the puzzle
        let mut the_locks = Vec::new();
        let mut the_keys = Vec::new();
        let mut row = 0;
        let mut locks = 0;
        let mut keys = 0;
        let mut corrupt = 0;
        let mut is_lock = true;
        let mut temp = 0u32;
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    if l.is_empty() {
                        row = 0;
                        temp = 0;
                        continue;
                    }
                    if row == 0 {
                        if l.eq("#####") {
                            locks += 1;
                            is_lock = true;
                        } else if l.eq(".....") {
                            keys += 1;
                            is_lock = false;
                        } else {
                            corrupt += 1;
                        }
                    } else {
                        if row == 6 {
                            if is_lock {
                                if l.eq(".....") {
                                    the_locks.push(temp);
                                } else {
                                    locks -= 1;
                                    corrupt += 1;
                                }
                                continue;
                            } else {
                                if l.eq("#####") {
                                    the_keys.push(temp);
                                } else {
                                    keys -= 1;
                                    corrupt += 1;
                                }
                                continue;
                            }
                        } else {
                            //
                            let mut tmp = 0;
                            for (i, c) in l.chars().enumerate() {
                                if c == '.' {
                                    continue;
                                } else if c == '#' {
                                    tmp += 1 << (4 - i) * 4;
                                } else {
                                    corrupt += 1;
                                }
                            }
                            temp += tmp;
                        }
                    }
                    row += 1;
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        println!("{}, {}, {}", locks, keys, corrupt);
        // println!("{}, {}", the_locks.len(), the_keys.len());
        const MASK: u32 = 0x88888;
        const BIAS: u32 = 0x22222;
        // println!("Lock 0: {}", format!("{:#07x}", the_locks[0]));
        // println!("Key  0: {}", format!("{:#07x}", the_keys[0]));
        let mut answer = 0;
        for lock in the_locks {
            for key in &the_keys {
                //
                if (BIAS + lock + *key) & MASK == 0 {
                    answer += 1;
                } else {
                    // println!("debug");
                    // println!("Lock   : {}", format!("{:#07x}", lock));
                    // println!("Key    : {}", format!("{:#07x}", key));
                    // println!("Sum    : {}", format!("{:#07x}", BIAS + lock + *key));
                    // println!("Masked : {}", format!("{:#07x}", (BIAS + lock + *key) & MASK));
                }
            }
        }
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

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
