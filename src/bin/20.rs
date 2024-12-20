use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::collections::HashSet;

const DAY: &str = "20"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"; // TODOne: Add the test input
const NORTH: (isize, isize) = (-1,0);
const EAST: (isize, isize) = (0,1);
const SOUTH: (isize, isize) = (1,0);
const WEST: (isize, isize) = (0,-1);
const DIRS: [(isize, isize); 4] = [NORTH, EAST, SOUTH, WEST];
#[derive(Debug, Clone)]
struct Square {
    c: char,
    score: isize,
    explored: bool,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<isize> {
        // TODOne: Solve Part 1 of the puzzle
        let to_dos: &mut Vec<(isize, usize, _)> = &mut Vec::new();
        let un_dos: &mut Vec<(isize, usize, _)> = &mut Vec::new();
        let grid: &mut Vec<Vec<char>> = &mut Vec::new();
        let sq_grid: &mut Vec<Vec<Square>> = &mut Vec::new();
        let bk_grid: &mut Vec<Vec<Square>> = &mut Vec::new();
        let mut walls: HashSet<(usize, usize)> = HashSet::new();
        let mut goal: (usize, usize) = (0,0);
        let mut start: (usize, usize) = (0,0);
        for (_y, line) in reader.lines().enumerate() {
            let lin = line?;
            let mut row = Vec::new();
            let mut g_row = Vec::new();
            let mut b_row = Vec::new();
            for (_x, c) in lin.chars().enumerate() {
                row.push(c);
                // all.insert((y, x));
                let sq = Square {
                    c,
                    score: 0,
                    explored: (c == '#'),
                };
                if c == 'S' {
                    to_dos.push((0, 0, (_y, _x)));
                    start = (start.0 + _y, start.1 + _x);
                }
                if c == 'E' {
                    un_dos.push((0, 0, (_y, _x)));
                    goal = (goal.0 + _y, goal.1 + _x);
                }
                if c == '#' {
                    walls.insert((_y, _x));
                }
                b_row.push(sq.clone());
                g_row.push(sq);
            }
            grid.push(row);
            sq_grid.push(g_row);
            bk_grid.push(b_row);
        }
        let xsz = grid[0].len();
        let ysz = grid.len();
        let wsz = walls.len();
        println!("Size:({xsz},{ysz}), start:{start:?}, goal:{goal:?}");
        println!("Walltiles: {wsz}");

        loop {
            let sq = to_dos.pop().unwrap();
            let y = sq.2.0;
            let x = sq.2.1;
            let score = sq.0;
            for (yy, xx) in DIRS {
                let yc = (y as isize + yy) as usize;
                let xc = (x as isize + xx) as usize;
                if sq_grid[yc][xc].explored {
                    continue;
                }
                to_dos.push((score + 1, 0, (yc, xc)));
            }
            sq_grid[y][x].explored = true;
            sq_grid[y][x].score = score;
            if sq_grid[y][x].c == 'E' || to_dos.is_empty() {
                break;
            }
            to_dos.sort_by(|a, b| b.0.cmp(&a.0));
        }
        loop {
            let sq = un_dos.pop().unwrap();
            let y = sq.2.0;
            let x = sq.2.1;
            let score = sq.0;
            for (yy, xx) in DIRS {
                let yc = (y as isize + yy) as usize;
                let xc = (x as isize + xx) as usize;
                if bk_grid[yc][xc].explored {
                    continue;
                }
                un_dos.push((score + 1, 0, (yc, xc)));
            }
            bk_grid[y][x].explored = true;
            bk_grid[y][x].score = score;
            if sq_grid[y][x].c == 'S' || un_dos.is_empty() {
                break;
            }
            un_dos.sort_by(|a, b| b.0.cmp(&a.0));
        }
        let min_score = sq_grid[goal.0][goal.1].score;
        println!("Forw: {}", min_score);
        let min_score_back = bk_grid[start.0][start.1].score;
        println!("Back: {}", min_score_back);

        let mut ctr = 0;
        for (y, x) in walls {
            if y == 0 || x == 0 || y == ysz - 1 || x == xsz - 1 {
                continue;
            }
            for df in DIRS {
                let yc = (y as isize + df.0) as usize;
                let xc = (x as isize + df.1) as usize;
                if sq_grid[yc][xc].c == '#' {
                    continue;
                }
                for db in DIRS {
                    if db == df {
                        continue;
                    }
                    let yb = (y as isize + db.0) as usize;
                    let xb = (x as isize + db.1) as usize;
                    if bk_grid[yb][xb].c == '#' {
                        continue;
                    }
                    if  bk_grid[yb][xb].score + sq_grid[yc][xc].score + 2 + 99>= min_score {
                        continue;
                    }
                    ctr += 1;
                }
            }
        }
        println!();
        println!("{ctr}");
        let answer = ctr;
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, limit: isize) -> Result<isize> {
        let to_dos: &mut Vec<(isize, usize, _)> = &mut Vec::new();
        let un_dos: &mut Vec<(isize, usize, _)> = &mut Vec::new();
        let grid: &mut Vec<Vec<char>> = &mut Vec::new();
        let sq_grid: &mut Vec<Vec<Square>> = &mut Vec::new();
        let bk_grid: &mut Vec<Vec<Square>> = &mut Vec::new();
        let mut all: HashSet<(usize, usize)> = HashSet::new();
        let mut goal: (usize, usize) = (0,0);
        let mut start: (usize, usize) = (0,0);
        for (_y, line) in reader.lines().enumerate() {
            let lin = line?;
            let mut row = Vec::new();
            let mut g_row = Vec::new();
            let mut b_row = Vec::new();
            for (_x, c) in lin.chars().enumerate() {
                row.push(c);
                all.insert((_y, _x));
                let sq = Square {
                    c,
                    score: 0,
                    explored: (c == '#'),
                };
                if c == 'S' {
                    to_dos.push((0, 0, (_y, _x)));
                    start = (start.0 + _y, start.1 + _x);
                }
                if c == 'E' {
                    un_dos.push((0, 0, (_y, _x)));
                    goal = (goal.0 + _y, goal.1 + _x);
                }
                b_row.push(sq.clone());
                g_row.push(sq);
            }
            grid.push(row);
            sq_grid.push(g_row);
            bk_grid.push(b_row);
        }
        let xsz = grid[0].len();
        let ysz = grid.len();

        loop {
            let sq = to_dos.pop().unwrap();
            let y = sq.2.0;
            let x = sq.2.1;
            let score = sq.0;
            for (yy, xx) in DIRS {
                let yc = (y as isize + yy) as usize;
                let xc = (x as isize + xx) as usize;
                if sq_grid[yc][xc].explored {
                    continue;
                }
                to_dos.push((score + 1, 0, (yc, xc)));
            }
            sq_grid[y][x].explored = true;
            sq_grid[y][x].score = score;
            if sq_grid[y][x].c == 'E' || to_dos.is_empty() {
                break;
            }
            to_dos.sort_by(|a, b| b.0.cmp(&a.0));
        }
        loop {
            let sq = un_dos.pop().unwrap();
            let y = sq.2.0;
            let x = sq.2.1;
            let score = sq.0;
            for (yy, xx) in DIRS {
                let yc = (y as isize + yy) as usize;
                let xc = (x as isize + xx) as usize;
                if bk_grid[yc][xc].explored {
                    continue;
                }
                un_dos.push((score + 1, 0, (yc, xc)));
            }
            bk_grid[y][x].explored = true;
            bk_grid[y][x].score = score;
            if sq_grid[y][x].c == 'S' || un_dos.is_empty() {
                break;
            }
            un_dos.sort_by(|a, b| b.0.cmp(&a.0));
        }
        let min_score = sq_grid[goal.0][goal.1].score;
        let _min_score_back = bk_grid[start.0][start.1].score;

        let mut cheats: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
        for (y, x) in all {
            if y == 0 || x == 0 || y == ysz - 1 || x == xsz - 1 {
                continue;
            }
            for df in DIRS {
                let yc = (y as isize + df.0) as usize;
                let xc = (x as isize + df.1) as usize;
                if sq_grid[yc][xc].c == '#' {
                    continue;
                }
                for db in DIRS {
                    if db == df {
                        continue;
                    }
                    const MAXLEN: isize = 20;
                    for ro in 1..MAXLEN
                    {
                        for ri in (ro-MAXLEN+1)..=(MAXLEN-1-ro) {
                            let mut yb = (y as isize + ro * db.0) as usize;
                            let mut xb = (x as isize + ro * db.1) as usize;
                            if 0 == db.0 {
                                yb = (y as isize + db.0 + ri) as usize;
                            }
                            if 0 == db.1 {
                                xb = (x as isize + db.1) as usize;
                            }
                            if yb >= ysz || xb >= xsz {
                                continue;
                            }
                            let d = ro + ri.abs();
                            if bk_grid[yb][xb].c == '#' {
                                continue;
                            }
                            if bk_grid[yb][xb].score + sq_grid[yc][xc].score + 1+d + limit - 1 >= min_score {
                                continue;
                            }
                            cheats.insert(((yc,xc),(yb,xb)));
                        }
                    }
                }
            }
        }
        println!("{}", cheats.len());
        let answer = cheats.len() as isize;
        // let mut test = isize::MIN;
        // for ((ya,xa),(yb,xb)) in cheats {
        //     let dy = (ya as isize - yb as isize).abs();
        //     let dx = (xa as isize - xb as isize).abs();
        //     if dy+dx > test {
        //         test = dy+dx;
        //     }
        // }
        // println!("Debug test: {test}");
        Ok(answer)
    }

    assert_eq!(32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
               part2(BufReader::new(TEST.as_bytes()), 50)?);
    // assert_eq!(31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
    //            part2(BufReader::new(TEST.as_bytes()), 52)?);
    // assert_eq!(29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
    //            part2(BufReader::new(TEST.as_bytes()), 54)?);
    // assert_eq!(39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
    //            part2(BufReader::new(TEST.as_bytes()), 56)?);
    // assert_eq!(25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
    //            part2(BufReader::new(TEST.as_bytes()), 58)?);
    // assert_eq!(23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
    //            part2(BufReader::new(TEST.as_bytes()), 60)?);
    // assert_eq!(20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
    //            part2(BufReader::new(TEST.as_bytes()), 62)?);
    // assert_eq!(3, part2(BufReader::new(TEST.as_bytes()), 76)?);
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()), 77)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 100)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
// src\bin\20.rs:341 took 10.4867628s.
// Result = 1012821