use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::ops::Add;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

struct Guard {
    x: usize,
    y: usize,
    d: usize,
}

const DAY: &str = "06"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODOne: Solve Part 1 of the puzzle
        let mut done = false;
        let looping = false;
        fn _print_grid(grid: &Vec<Vec<i8>>) -> () {
            for _test in grid.iter(){
                // println!("{}", _test.len());
                for _s in _test.iter(){
                    if *_s < 0i8 {
                        print!("#");
                    } else {
                        print!("{}", *_s as i32);
                    }
                }
                println!();
            }
        }
        let grid: &mut Vec<Vec<i8>> = &mut Vec::new();
        let mut guard_x = 0;
        let mut guard_y = 0;
        let guard_d = 0;
        for (y, line) in reader.lines().enumerate() {
            // println!("{}", line?);
            let lin = line?;
            // let lie = lin.trim();
            let mut row = Vec::new();
            for (x, c) in lin.chars().enumerate() {
                // println!("{x}");
                // print!("{c}");
                // println!("{}", row.len());
                if c == '#' {
                    row.push(-128i8);
                }
                else if c == '^' {
                    row.push(1i8);
                    guard_x = x;
                    guard_y = y;
                }
                else if c == '.' {
                    row.push(0i8);
                } else {
                    println!("ELSE!");
                }
                // println!("{}", row.len());
            }
            grid.push(row);
            // println!();
        }
        let ysz = grid.len();
        let xsz = grid[0].len();
        println!("{xsz},{ysz}");
        println!("{guard_x},{guard_y}");
        let guard: &mut Guard = &mut Guard{ x: guard_x, y: guard_y, d: guard_d };

        fn moveg(grid: &mut Vec<Vec<i8>>, guard: &mut Guard, done: &mut bool, xysz: usize) -> bool {
            fn mov_n(grid: &mut Vec<Vec<i8>>, guard: &mut Guard, done: &mut bool, xysz: usize) -> bool {
                // println!("N guard at ({}, {}) {}", guard.x, guard.y, guard.d);
                let gx = guard.x;
                // println!("dbg guard.y is: {}", guard.y);
                let gy = guard.y.checked_sub(1);
                if gy.is_none(){
                    println!("exit north!");
                    *done = true;
                    return false;
                }
                let gy = gy.unwrap();

                    // println!("geeyyy {gy}");

                    let g_new = grid[gy][gx];
                    if g_new >= 0i8 {
                        // println!("moving north...");
                        let row = grid.get_mut(gy);
                        let row = row.unwrap();
                        row[gx] |= 1i8;
                        grid[gy] = row.to_vec();
                        guard.y =gy;
                        return true
                    }else {
                        //
                        // println!("fail!");
                        return false;
                    }
            }// mov_n

            fn mov_e(grid: &mut Vec<Vec<i8>>, guard: &mut Guard, done: &mut bool, xsz: usize) -> bool {
                let gy = guard.y;
                // println!("dbg guard.y is: {}", guard.y);
                let gx = guard.x.checked_add(1);
                let gx = gx.unwrap();
                // guard.x = gx;
                // println!("geexxx {gx}");

                let g_new = grid[gy][gx];
                if g_new >= 0i8 {
                    // println!("moving east...");
                    let row = grid.get_mut(gy);
                    let row = row.unwrap();
                    row[gx] |= 1i8;
                    grid[gy] = row.to_vec();
                    guard.x = gx;
                    return true
                }else {
                    //
                    // println!("fail!");
                    return false;
                }
            }// mow_e

            fn mov_s(grid: &mut Vec<Vec<i8>>, guard: &mut Guard, done: &mut bool, ysz: usize) -> bool {
                let gx = &guard.x;
                // println!("dbg guard.y is: {}", guard.y);
                let gy = guard.y.checked_add(1);
                let gy = gy.unwrap();
                if gy >= ysz{
                    *done = true;
                    println!("exit south!");
                    return false;
                }

                // println!("geeyyy {gy}");

                let g_new = grid[gy][*gx];
                if g_new >= 0i8 {
                    // println!("moving south...");
                    let row = grid.get_mut(gy);
                    let row = row.unwrap();
                    row[*gx] |= 1i8;
                    grid[gy] = row.to_vec();
                    guard.y =gy;
                    return true
                }else {
                    //
                    // println!("fail!");
                    return false;
                }
            }// mov_s

            fn mov_w(grid: &mut Vec<Vec<i8>>, guard: &mut Guard, done: &mut bool, xysz: usize) -> bool {
                let gy = guard.y;
                // println!("dbg guard.y is: {}", guard.y);
                let gx = guard.x.checked_sub(1);
                let gx = gx.unwrap();

                // println!("geexxx {gx}");

                let g_new = grid[gy][gx];
                if g_new >= 0i8 {
                    // println!("moving west...");
                    let row = grid.get_mut(gy);
                    let row = row.unwrap();
                    row[gx] |= 1i8;
                    grid[gy] = row.to_vec();
                    guard.x =gx;
                    return true
                }else {
                    //
                    // println!("fail!");
                    return false;
                }
            }// mow_w

            // print!("guard ");
            // println!("...{} {}", &guard.d, guard.d == 0);
            if guard.d == 0 {
                if !mov_n(grid, guard, done, xysz){
                    guard.d += 1;
                    return mov_e(grid, guard, done, xysz);
                } else {
                    return true;
                }
            } else if guard.d == 1 {
                if !mov_e(grid, guard, done, xysz){
                    guard.d += 1;
                    return mov_s(grid, guard, done,xysz);
                } else {
                    return true;
                }
            } else if guard.d == 2 {
                if !mov_s(grid, guard, done, xysz){
                    if *done{
                        return false;
                    }
                    guard.d += 1;
                    return mov_w(grid, guard, done, xysz);
                } else {
                    return true;
                }
            } else if guard.d == 3 {
                if !mov_w(grid, guard, done, xysz){
                    guard.d = 0;
                    return mov_n(grid, guard, done, xysz);
                } else {
                    return true;
                }
            }
            false
        }
        let mut mov_cnt: u16 = 0;
        // for _d in 0..5950 {
        for _d in 0..16384 {
            //
            //print_grid(grid);
            // println!("Guard at ({}, {}) {}", guard.x, guard.y, guard.d);
            if moveg(grid, guard, &mut done, xsz) {
                // println!("debug is true true? a {}", mov_cnt);
                mov_cnt += 1;
                // println!("debug is true true? b {}", mov_cnt);
            } else {
                break;
            }
            // print_grid(&grid);
            // println!("moves: {mov_cnt}, {_d}");
        }
        let mut answer = 0;
        for row in grid{
            for s in row{
                if *s > 0 {
                    // print!("y ");
                    answer += 1;
                }
            }
        }
        // reader.lines().flatten().count();
        println!("Moves: {}", mov_cnt);
        println!("Done: {}", done);
        println!("Looping: {}", looping);
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

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
// Result = 5409 (@5550) OK!
//