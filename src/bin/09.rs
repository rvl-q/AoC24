// use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
// use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "09"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
"; // TODOne: Add the test input

#[derive(Debug)]
struct Block {
    id: usize,
    part: usize,
    used: bool,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        // TODOne: Solve Part 1 of the puzzle
        let mut blocks: Vec<_> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            println!("{}", line.len());
            for (i, c) in line.chars().enumerate(){
                let n = c.to_digit(10).unwrap() as usize;
                for j in 0..n {
                    // println!("{i}, {j}");

                    if i % 2 == 0 {
                        let b = Block { id: i / 2, part: j, used: true };
                        // println!("{:?}", &b);
                        blocks.push(b);
                    } else {
                        let b = Block { id: 0, part: j, used: false };
                        // println!("{:?}", &b);
                        blocks.push(b);
                    }
                }
                // print!("{n} ");
            }
        }
        println!("{:?}",blocks.len());
        fn _print_blocks(blocks: &Vec<Block>) {
            for b in blocks {
                if b.used {
                    print!("{}", b.id);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        fn block_sum(blocks: &Vec<Block>) -> u64 {
            let mut chk_sum = 0u64;
            for (i, b) in blocks.iter().enumerate() {
                if b.used {
                    chk_sum = chk_sum + (i * b.id) as u64;
                }
            }
            chk_sum
        }

        fn first_empty(blocks: &Vec<Block>, first: usize) -> usize {
            // let first = 1000000;
            // println!("{first}");
            // for (i, b) in blocks.iter().enumerate() {
            //     if !b.used {
            //         return i;
            //     }
            // }
            for i in first..blocks.len() {
                if !blocks[i].used {
                    return i;
                }
            }
            // never actually going here ...I hope
            1000000
        }

        fn last_used(blocks: &Vec<Block>, last: usize) -> usize {
            // let sze = blocks.len();
            // println!("{last}");
            for i in (0..last).rev() {
                if blocks[i].used {
                    return i;
                }
            }
            println!("what???");
            0
        }

        fn mov_blc(blocks: &mut Vec<Block>, src: usize, dst: usize) -> () {
            let srce_id = blocks[src].id;
            let srce_part = blocks[src].part;
            let dest = &mut blocks[dst];
            // println!("dbg");
            dest.id = srce_id;
            dest.part = srce_part;
            dest.used = true;
            blocks[src].used = false;
        }

        let mut first: usize = 0;
        let mut last: usize = blocks.len();
        loop {
            // print_blocks(&blocks);
            // let chk_sum = block_sum(&blocks);
            // println!("{chk_sum}");
            first = first_empty(&blocks, first);
            // println!("{first}");
            // println!("{last}");
            last = last_used(&blocks, last);
            // println!("{last}");
            if last > first {
                // move...
                mov_blc(&mut blocks, last, first);
            } else {
                break;
            }
        }
        // print_blocks(&blocks);

        let answer = block_sum(&blocks);
        println!("{answer}");
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut blocks: Vec<_> = Vec::new();
        // let mut free_blocks: HashMap<usize, Vec<usize>> = HashMap::new();
        // let mut idx: usize = 0;
        for line in reader.lines() {
            let line = line?;
            // println!("{}", line.len());
            for (i, c) in line.chars().enumerate(){
                let n = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    // Empty spaces
                    // what are we living for?
                }
                for j in 0..n {
                    // println!("{i}, {j}");

                    if i % 2 == 0 {
                        let b = Block { id: i / 2, part: j, used: true };
                        // println!("{:?}", &b);
                        blocks.push(b);
                    } else {
                        let b = Block { id: 0, part: j, used: false };
                        // println!("{:?}", &b);
                        blocks.push(b);
                    }
                }
                // print!("{n} ");
            }
        }
        // println!("{:?}",blocks.len());
        fn _print_blocks(blocks: &Vec<Block>) {
            for b in blocks {
                if b.used {
                    print!("{}", b.id);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        fn block_sum(blocks: &Vec<Block>) -> u64 {
            let mut chk_sum = 0u64;
            for (i, b) in blocks.iter().enumerate() {
                if b.used {
                    chk_sum = chk_sum + (i * b.id) as u64;
                }
            }
            chk_sum
        }

        fn first_empty(blocks: &Vec<Block>, first: usize) -> usize {
            for i in first..blocks.len() {
                if !blocks[i].used {
                    return i;
                }
            }
            // never actually going here ...I hope
            1000000
        }

        fn first_empty_sz(blocks: &Vec<Block>, first: usize, len: usize) -> usize {
            for mut i in first..blocks.len() {
                if !blocks[i].used {
                    let mut found = true;
                    for _j in 1..len{
                        i += 1;
                        let test = blocks.get(i);
                        if test.is_none(){
                            return 1000000;
                        }
                        let test = test.unwrap();
                        if test.used {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        // yes
                        // println!("Found at: {}", i - (len - 1));
                        return i + 1 - len;
                    }
                }
            }
            // not found
            1000000
        }

        fn last_used(blocks: &Vec<Block>, last: usize) -> usize {
            // let sze = blocks.len();
            // println!("{last}");
            for i in (0..last).rev() {
                if blocks[i].used {
                    return i;
                }
            }
            println!("what???");
            0
        }

        fn mov_blc(blocks: &mut Vec<Block>, src: usize, dst: usize) -> () {
            let srce_id = blocks[src].id;
            let srce_part = blocks[src].part;
            let dest = &mut blocks[dst];
            // println!("dbg");
            dest.id = srce_id;
            dest.part = srce_part;
            dest.used = true;
            blocks[src].used = false;
        }

        fn mov_blcs(blocks: &mut Vec<Block>, src: usize, dst: usize, sze: usize) -> () {
            for i in 0..sze {
                mov_blc(blocks, src + i, dst + i);
            }
        }

        let mut first: usize = 0;
        // let mut last: usize = blocks.len();

        // // test...
        // for i in 1..10usize {
        //     // test
        //     let res = first_empty_sz(&blocks, 0, i);
        //     println!("{i}: {res}");
        // }
        // return Ok(0);

        // // test mov_blcs
        // mov_blcs(&mut blocks, 32, 8, 3);
        // mov_blcs(&mut blocks, 15, 2, 3);

        // loop {
        //     _print_blocks(&blocks);
        //     first = first_empty(&blocks, first);
        //     last = last_used(&blocks, last);
        //     if last > first {
        //         mov_blc(&mut blocks, last, first);
        //     } else {
        //         break;
        //     }
        // }
        // // _print_blocks(&blocks);
        let mut prev: usize = blocks.len();
        loop {
            // _print_blocks(&blocks);
            prev = last_used(&blocks, prev);
            let len = blocks[prev].part + 1;
            prev -= blocks[prev].part;
            first = first_empty(&blocks, first);
            if len == 1 {
                if prev > first {
                    mov_blc(&mut blocks, prev, first);
                } else {
                    break;
                }
            } else {
                let has_space = first_empty_sz(&blocks, first, len);
                if has_space > prev {
                    continue;
                }
                mov_blcs(&mut blocks, prev, has_space, len);
            }
        }
        // _print_blocks(&blocks);

        let answer = block_sum(&blocks);
        Ok(answer)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
//
// src\bin\09.rs:144 took 3.5159ms.
// Result = 6346871685398
// === Part 2 ===
// src\bin\09.rs:323 took 196.0022ms.
// Result = 6373055193464
//