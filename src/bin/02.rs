use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // TODOne: Solve Part 1 of the puzzle
        let mut lines: Vec<Vec<i32>> = Vec::new();
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    let parts: Vec<&str> = l.split(" ").collect_vec();
                    //println!("{parts:?}");
                    let mut row: Vec<i32> = Vec::new();
                    for p in parts {
                        let n :i32 = p.parse()?;
                        row.push(n);
                    }
                    lines.push(row);
                    //
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }

        let mut answer = 0;
        for row in lines{
            let mut flag = 0i32;
            // println!("{row:?}");
            if row[0]-row[1] == 0 || (row[0]-row[1]).abs() > 3 {
                // println!("fail!");
                continue
            }
            if flag == 0 {
                if row[1] > row[0]{
                    flag = 1;
                } else {
                    flag = -1;
                }
            }
            if flag == 1 {
                let mut prod = 1;
                for i in 1..row.len(){
                    if row[i] - row[i-1] < 1 || row[i] - row[i-1] > 3 {
                        prod = 0;
                        break;
                    }
                }
                answer += prod;
            } else {
                let mut prod = 1;
                for i in 1..row.len(){
                    if row[i] - row[i-1] > -1 || row[i] - row[i-1] < -3 {
                        prod = 0;
                        break;
                    }
                }
                answer += prod;
            }
        }
        //reader.lines().flatten().count();
        println!("{answer}");
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn inc_ok(a: i32, b: i32) -> bool{
        b-a > 0 && b-a < 4
    }
    fn dec_ok(a: i32, b: i32) -> bool{
        a-b > 0 && a-b < 4
    }
    fn increasing(row: Vec<i32>) -> bool {
        //println!("{row:?}");
        for i in 1..row.len(){
            if !inc_ok(row[i-1], row[i]){
                return false;
            }
        }
        true
    }
    fn decreasing(row: Vec<i32>) -> bool {
        //println!("{row:?}");
        for i in 1..row.len(){
            if !dec_ok(row[i-1], row[i]){
                return false;
            }
        }
        true
    }

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut lines: Vec<Vec<i32>> = Vec::new();
        for line in reader.lines() {
            match line {
                std::result::Result::Ok(l) => {
                    let parts: Vec<&str> = l.split(" ").collect_vec();
                    //println!("{parts:?}");
                    let mut row: Vec<i32> = Vec::new();
                    for p in parts {
                        let n :i32 = p.parse()?;
                        row.push(n);
                    }
                    lines.push(row);
                    //
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }

        let mut answer = 0;
        let mut bads = 0;
        let mut bad0 = 0;
        let mut badi = 0;
        let mut badd = 0;
        let mut max = 0usize;
        let mut min = usize::MAX;
        for row in lines{
            let mut flag;
            let mut incs = 0;
            let mut decs = 0;
            for i in 1..row.len() {
                if inc_ok(row[i-1], row[i]) {
                    incs += 1;
                } else if dec_ok(row[i-1], row[i]) {
                    decs += 1;
                }
            }
            flag = incs - decs;
            if flag > 0 {
                flag = 1; // majority increasing
            } else if flag < 0 {
                flag = -1; // majority decreasing
            }
            // println!("{row:?}: ^:{incs}, v:{decs}");
            if row.len() < min {
                min = row.len()
            }
            if row.len() > max {
                max = row.len()
            }
            // println!("{row:?}");
            if row[0]-row[1] == 0 || (row[0]-row[1]).abs() > 3 {
                //println!("fail!? {row:?}");
                let (head, tail) = row.split_at(2);
                // println!("{row:?}:\n{head:?} .. {tail:?}");
                if increasing(tail.to_vec()){
                    // println!("Increasing OK!");
                    // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    if inc_ok(head[0], tail[0]) || inc_ok(head[1], tail[0]){
                        // println!("Inc, yes!");
                        // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                        answer += 1;
                        continue;
                    }
                }else if decreasing(tail.to_vec()){
                    // println!("Decreasing OK!");
                    // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    if dec_ok(head[0], tail[0]) || dec_ok(head[1], tail[0]) {
                        // println!("Dec, yes!");
                        // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                        answer += 1;
                        continue;
                    }
                } else {
                    //?continue;
                    bads += 1;
                    bad0 += 1;
                    // println!("Fail 0 {row:?}");
                    continue;
                }
            }
            // if flag == 0 {
            //     if row[1] > row[0]{
            //         flag = 1;
            //     } else {
            //         flag = -1;
            //     }
            // }
            if flag == 1 {
                if increasing(row.clone()){
                    // println!("Row: {row:?}");
                    // assert_eq!(1,0);
                    answer += 1;
                } else {
                    //brute
                    let mut okay = false;
                    for i in 0..row.len() {
                        let mut test: Vec<_> = Vec::new();
                        for j in 0..row.len() {
                            if i == j {
                                continue;
                            }
                            test.push(row[j]);
                        }
                        if increasing(test.clone()){
                            // println!("Row: {row:?} Pass: {test:?}");
                            // assert_eq!(1,0);
                            answer += 1;
                            okay = true;
                            break;
                        }
                    }
                    // let (head, tail) = row.split_at(2);
                    // if increasing(tail.to_vec()){
                    //     // println!("Increasing OK!");
                    //     // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    //     if inc_ok(head[0], tail[0]) || inc_ok(head[1], tail[0]){
                    //         // println!("Inc B, yes!");
                    //         // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    //         answer += 1;
                    //         continue;
                    //     }
                    // // }else if decreasing(tail.to_vec()){
                    // //     // println!("Decreasing OK!");
                    // //     // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    // //     if dec_ok(head[0], tail[0]) || dec_ok(head[1], tail[0]) {
                    // //         // println!("Dec B, yes!");
                    // //         // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    // //         answer += 1;
                    // //         continue;
                    // //     }
                    // }
                    // // now increasing or fail
                    // let mut okay = true;
                    // let mut once = false;
                    // let mut brute = head.to_vec().clone();
                    // for i in 0..tail.len() {
                    //     brute.push(tail[i]);
                    //     if increasing(brute.clone()) {
                    //         // println!("debug!");
                    //         // println!("Brute: {brute:?}, {row:?}");
                    //     } else {
                    //         brute.pop();
                    //         if once{
                    //             // println!("fail!");
                    //             okay = false;
                    //             bads += 1;
                    //             badi += 1;
                    //             // println!("Fail? inc {row:?}");
                    //             break;
                    //         }
                    //         once = true;
                    //     }
                    // }
                    // if okay{
                    //     answer += 1;
                    //     println!("Pass! ^ {row:?}")
                    // }
                    if !okay{
                        badi += 1;
                        bads += 1;
                    }
                }
            } else {
                if decreasing(row.clone()){
                    answer += 1;//1
                } else {
                    //brute
                    let mut okay = false;
                    for i in 0..row.len() {
                        let mut test: Vec<_> = Vec::new();
                        for j in 0..row.len() {
                            if i == j {
                                continue;
                            }
                            test.push(row[j]);
                        }
                        if decreasing(test.clone()) {
                            // println!("Row: {row:?} Pass: {test:?}");
                            // assert_eq!(1,0);
                            answer += 1;
                            okay = true;
                            break;
                        }
                    }
                    if !okay {
                        badd += 1;
                        bads += 1;
                    }

                    // //decreasing or early error
                    // let (head, tail) = row.split_at(2);
                    // // if increasing(tail.to_vec()){
                    // //     // println!("Increasing OK!");
                    // //     // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    // //     if inc_ok(head[0], tail[0]) || inc_ok(head[1], tail[0]){
                    // //         // println!("Inc B, yes!");
                    // //         // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    // //         answer += 1;
                    // //         continue;
                    // //     }
                    // // }else
                    // if decreasing(tail.to_vec()){
                    //     // println!("Decreasing OK!");
                    //     // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    //     if dec_ok(head[0], tail[0]) || dec_ok(head[1], tail[0]) {
                    //         // println!("Dec B, yes!");
                    //         // println!("head: {}, {} & {}", head[0], head[1], tail[0]);
                    //         answer += 1;
                    //         continue;
                    //     }
                    // }
                    // // now decreasing or fail
                    // let mut okay = true;
                    // let mut once = false;
                    // let mut brute = head.to_vec().clone();
                    // for i in 0..tail.len() {
                    //     brute.push(tail[i]);
                    //     if decreasing(brute.clone()) {
                    //         // println!("debug!");
                    //         // println!("Brute: {brute:?}, {row:?}");
                    //     } else {
                    //         brute.pop();
                    //         if once{
                    //             // println!("fail!");
                    //             okay = false;
                    //             bads += 1;
                    //             badd += 1;
                    //             // println!("Fail? dec {row:?}");
                    //             break;
                    //         }
                    //         once = true;
                    //     }
                    // }
                    // if okay{
                    //     answer += 1;
                    //     println!("Pass! v {row:?}");
                    // }
                }
            }
        }
        println!("Min & Max lengths: {min}, {max}");
        println!("Bads: {bads} = {bad0} + {badi} + {badd}");
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
