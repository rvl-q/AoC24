use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;
use std::cmp::*;
// use std::collections::HashMap;

const DAY: &str = "13"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // TODOne: Solve Part 1 of the puzzle
        let mut counter = -1;
        let mut ax: Vec<i32> = Vec::new();
        let mut bx: Vec<i32> = Vec::new();
        let mut ay: Vec<i32> = Vec::new();
        let mut by: Vec<i32> = Vec::new();
        let mut xx: Vec<i32> = Vec::new();
        let mut yy: Vec<i32> = Vec::new();
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    if l.is_empty() {
                        continue;
                    }
                    counter = (counter + 1) % 3;
                    if counter == 0 {
                        let parts: Vec<&str> = l.split(", ").collect_vec();
                        // println!("{parts:?}");
                        let px:i32 = parts[0].split("X+").collect_vec()[1].parse()?;
                        let py: i32 = parts[1].split("Y+").collect_vec()[1].parse()?;
                        // println!("A: {px}, {py}");
                        ax.push(px);
                        ay.push(py);
                        //
                    } else if counter == 1{
                        let parts: Vec<&str> = l.split(", ").collect_vec();
                        let px:i32 = parts[0].split("X+").collect_vec()[1].parse()?;
                        let py: i32 = parts[1].split("Y+").collect_vec()[1].parse()?;
                        // println!("B: {px}, {py}");
                        bx.push(px);
                        by.push(py);
                    } else {
                        let parts: Vec<&str> = l.split(", ").collect_vec();
                        let px:i32 = parts[0].split("X=").collect_vec()[1].parse()?;
                        let py: i32 = parts[1].split("Y=").collect_vec()[1].parse()?;
                        // println!("P: {px}, {py}");
                        xx.push(px);
                        yy.push(py);
                    }
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        // println!("{ax:?}");
        // println!("{ay:?}");
        // println!("{bx:?}");
        // println!("{by:?}");
        // println!("{xx:?}");
        // println!("{yy:?}");
        let n = ax.len();
        println!("{n}");
        let mut answer = 0;
        for m in 0..n {
            // println!("{},{},{},{},{},{}",ax[m],ay[m],bx[m],by[m],xx[m],yy[m]);
            let b1 =xx[m] / bx[m];
            let b2 =yy[m] / by[m];
            let mut b = min(b1, b2);
            b = min(100, b);
            // println!("{b1},{b2}: {b}, {}", b<=100);
            let a_x = ax[m];
            let a_y = ay[m];
            let b_x = bx[m];
            let b_y = by[m];
            let x_x = xx[m];
            let y_y = yy[m];
            let mut a = (x_x - b * b_x) / a_x;
            let mut dx = a * a_x + b * b_x - x_x;
            let mut dy = a * a_y + b * b_y - y_y;
            loop {
                if dx == 0 && dy == 0 {
                    // println!("Yes! {a},{b}");
                    // push..
                    answer += 3*a;
                    answer += b;
                    break;
                }
                b -= 1;
                if b < 0 {
                    // println!("NO! {a},{b}");
                    // println!("{},{},{},{},{},{}",ax[m],ay[m],bx[m],by[m],xx[m],yy[m]);
                    // println!("dbg");
                    break;
                }
                a = (x_x - b * b_x) / a_x;
                dx = a * a_x + b * b_x - x_x;
                dy = a * a_y + b * b_y - y_y;
            }

            // if m == 4 {
            //     break;
            // }
        }

        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    const PART2: i64 = 10000000000000;
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let mut counter = -1i64;
        let mut ax: Vec<i32> = Vec::new();
        let mut bx: Vec<i32> = Vec::new();
        let mut ay: Vec<i32> = Vec::new();
        let mut by: Vec<i32> = Vec::new();
        let mut xx: Vec<i32> = Vec::new();
        let mut yy: Vec<i32> = Vec::new();
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    if l.is_empty() {
                        continue;
                    }
                    counter = (counter + 1) % 3;
                    if counter == 0 {
                        let parts: Vec<&str> = l.split(", ").collect_vec();
                        // println!("{parts:?}");
                        let px:i32 = parts[0].split("X+").collect_vec()[1].parse()?;
                        let py: i32 = parts[1].split("Y+").collect_vec()[1].parse()?;
                        // println!("A: {px}, {py}");
                        ax.push(px);
                        ay.push(py);
                        //
                    } else if counter == 1{
                        let parts: Vec<&str> = l.split(", ").collect_vec();
                        let px:i32 = parts[0].split("X+").collect_vec()[1].parse()?;
                        let py: i32 = parts[1].split("Y+").collect_vec()[1].parse()?;
                        // println!("B: {px}, {py}");
                        bx.push(px);
                        by.push(py);
                    } else {
                        let parts: Vec<&str> = l.split(", ").collect_vec();
                        let px:i32 = parts[0].split("X=").collect_vec()[1].parse()?;
                        let py: i32 = parts[1].split("Y=").collect_vec()[1].parse()?;
                        // println!("P: {px}, {py}");
                        xx.push(px);
                        yy.push(py);
                    }
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        // println!("{ax:?}");
        // println!("{ay:?}");
        // println!("{bx:?}");
        // println!("{by:?}");
        // println!("{xx:?}");
        // println!("{yy:?}");
        let n = ax.len();
        println!("{n}");
        let mut answer = 0i64;
        for m in 0..n {
            let a_x = ax[m] as i64;
            let a_y = ay[m] as i64;
            let b_x = bx[m] as i64;
            let b_y = by[m] as i64;
            let x_x = xx[m] as i64 + PART2;
            let y_y = yy[m] as i64 + PART2;
            println!("{},{},{},{},{},{}",a_x,a_y,b_x,b_y,x_x,y_y);
            let b1 =x_x / b_x;
            let b2 =y_y / b_y;
            let mut b = min(b1, b2);
            // b = min(100, b);
            // println!("{b1},{b2}: {b}, {}", b<=100);
            let mut a = (x_x - b * b_x) / a_x;
            let mut dx = a * a_x + b * b_x - x_x;
            let mut dy = a * a_y + b * b_y - y_y;
            let mut step = 1;
            let mut ctr = 0;
            let mut stp_x_fnd = false;
            let mut old_cnt = -1i64;
            let mut dy_old = 0i64;
            loop {
                ctr += 1;
                if dx == 0 && dy == 0 {
                    println!("Yes! {a},{b} ctr:{ctr}");
                    // push..
                    answer += 3*a;
                    answer += b;
                    break;
                }
                if ctr > 1000 {
                    println!("NO NO NO!!! {a},{b} ctr:{ctr}");
                    break;
                }
                if dx == 0 {
                    if ! stp_x_fnd {
                        println!("Yes, x: {a},{b} ctr:{ctr} and dy={dy}, ddy={}", dy-dy_old);
                        if old_cnt > 0 {
                            step = ctr - old_cnt;
                            stp_x_fnd = true;
                            let jump = dy / (dy_old - dy);
                            println!("Jump: {jump}");
                            println!("Jump * step: {}", jump*step);
                            if jump < 10 {
                                println!("alarrrm!");
                            } else {
                                b -= (jump-1) * step;
                            }
                        }
                        dy_old = dy;
                        old_cnt = ctr;
                    } else {
                        if dy.abs() > dy_old.abs() {
                            println!("NO? {a},{b} dy: {dy} ctr:{ctr}");
                            break;
                        }
                        dy_old = dy;
                    }
                    // push..
                }
                if dy == 0 {
                    println!("Yes, y {a},{b} ctr:{ctr}");
                    // push..
                }
                b -= step;
                if b < 0 {
                    println!("NO! {a},{b} ctr:{ctr}");
                    // println!("{},{},{},{},{},{}",ax[m],ay[m],bx[m],by[m],xx[m],yy[m]);
                    // println!("dbg");
                    break;
                }
                a = (x_x - b * b_x) / a_x;
                dx = a * a_x + b * b_x - x_x;
                dy = a * a_y + b * b_y - y_y;
            }

            // if m == 4 {
            //     break;
            // }
        }

        Ok(answer)
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
