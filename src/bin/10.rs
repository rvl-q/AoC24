use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "10"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"; // TODOne: Add the test input

const GOAL: i8 = 9;

#[derive(Debug, Clone)]
struct Square {
    height: i8,
    y: usize,
    x: usize,
    explored: bool,
    finals: HashSet<(usize, usize)>,
}
#[derive(Debug, Clone)]
struct Square2 {
    height: i8,
    y: usize,
    x: usize,
    explored: bool,
    finals: HashSet<(usize, usize)>,
    paths: usize,
}


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODOne: Solve Part 1 of the puzzle
        let mut trailheads: Vec<(usize, usize)> = Vec::new();
        let mut levels: HashMap<i8, Vec<(usize, usize)>> = HashMap::new();
        for i in 0..9 {
            levels.insert(i, Vec::new());
        }
        let mut grid: HashMap<(usize, usize), Square> = HashMap::new();
        let mut y = 0;
        let mut side_len = 0;
        for line in reader.lines() {
            let line = line?;
            if y == 0 {
                side_len = line.len();
                println!("{}", line.len());
            }
            for (x, c) in line.chars().enumerate() {
                let height = c.to_digit(10).unwrap() as i8;
                let mut sq = Square{
                    height,
                    y,
                    x,
                    explored: false,
                    finals: HashSet::new(),
                };
                if height == 0 {
                    trailheads.push((y, x));
                }
                if height == GOAL {
                    sq.explored = true;
                    sq.finals.insert((y, x));
                } else {
                    let level_n: &mut Vec<(usize, usize)> = &mut levels.get(&height).unwrap().to_owned();
                    level_n.push((y, x));
                    levels.insert(height, level_n.to_owned());
                }
                // suppress never used warning..
                let _dummy = sq.x;
                let _dummy = sq.y;
                grid.insert((y, x), sq);
            }
            y += 1;
        }
        let side_len = side_len;
        println!("{:?}",grid.len());
        // println!("{:?}",grid);
        // println!("{}: {:?}",trailheads.len(),trailheads);
        // let th = trailheads[0];
        // println!("{:?}", th);
        // let th = grid.get(&(th.0, th.1));
        // println!("{:?}", th);
        // for _kv in &levels {
        //     // println!("{_kv:?}")
        // }
        for lvl in (0..9).rev() {
            // level
            let level = &levels.get(&lvl).unwrap();
            for (y, x) in *level {
                // print!("{y}{x} ")
                let sq: &mut Square = &mut grid.get(&(*y, *x)).unwrap().to_owned();
                let mut neighbours: Vec<&Square> = Vec::new();
                if *y > 0 {
                    let nn = grid.get(&(*y-1,*x)).unwrap();
                    neighbours.push(&nn);
                }
                if *y < side_len-1 {
                    let ns = grid.get(&(*y+1,*x)).unwrap();
                    neighbours.push(ns);
                }
                if *x < side_len-1 {
                    let ne = grid.get(&(*y, *x + 1)).unwrap();
                    neighbours.push(ne);
                }
                if *x > 0 {
                    let nw = &grid.get(&(*y,*x - 1)).unwrap();
                    neighbours.push(nw);
                }
                for nbr in neighbours {
                    if nbr.height - sq.height == 1 {
                        sq.finals.extend(&nbr.finals);
                    }
                }
                sq.explored = true;
                let sq = sq.to_owned();
                grid.insert((*y,*x), sq);
            }
        }
        // println!("{grid:?}");

        let mut answer = 0;
        for (y, x) in trailheads {
            let sq = &grid.get(&(y, x)).unwrap();
            let sq = *sq;
            let ends = sq.finals.len();
            // println!("({x}, {y}): {ends}");
            answer += ends;
        }
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut trailheads: Vec<(usize, usize)> = Vec::new();
        let mut levels: HashMap<i8, Vec<(usize, usize)>> = HashMap::new();
        for i in 0..9 {
            levels.insert(i, Vec::new());
        }
        let mut grid: HashMap<(usize, usize), Square2> = HashMap::new();
        let mut y = 0;
        let mut side_len = 0;
        for line in reader.lines() {
            let line = line?;
            if y == 0 {
                side_len = line.len();
            }
            for (x, c) in line.chars().enumerate() {
                let height = c.to_digit(10).unwrap() as i8;
                let mut sq = Square2{
                    height,
                    y,
                    x,
                    explored: false,
                    finals: HashSet::new(),
                    paths: 0,
                };
                if height == 0 {
                    trailheads.push((y, x));
                }
                if height == GOAL {
                    sq.explored = true;
                    sq.finals.insert((y, x));
                    sq.paths = 1;
                } else {
                    let level_n: &mut Vec<(usize, usize)> = &mut levels.get(&height).unwrap().to_owned();
                    level_n.push((y, x));
                    levels.insert(height, level_n.to_owned());
                }
                let _dummy = sq.x;
                let _dummy = sq.y;
                grid.insert((y, x), sq);
            }
            y += 1;
        }
        let side_len = side_len;
        for lvl in (0..9).rev() {
            // level
            let level = &levels.get(&lvl).unwrap();
            for (y, x) in *level {
                let sq: &mut Square2 = &mut grid.get(&(*y, *x)).unwrap().to_owned();
                let mut neighours: Vec<&Square2> = Vec::new();
                if *y > 0 {
                    let nn = grid.get(&(*y-1,*x)).unwrap();
                    neighours.push(&nn);
                }
                if *y < side_len-1 {
                    let ns = grid.get(&(*y+1,*x)).unwrap();
                    neighours.push(ns);
                }
                if *x < side_len-1 {
                    let ne = grid.get(&(*y, *x + 1)).unwrap();
                    neighours.push(ne);
                }
                if *x > 0 {
                    let nw = &grid.get(&(*y,*x - 1)).unwrap();
                    neighours.push(nw);
                }
                for nbr in neighours {
                    if nbr.height - sq.height == 1 {
                        sq.finals.extend(&nbr.finals);
                        sq.paths += &nbr.paths;
                    }
                }
                sq.explored = true;
                // let sq: &Square = sq;
                let sq = sq.to_owned();
                grid.insert((*y,*x), sq);
            }
            // println!();
        }
        // println!("{grid:?}");

        let mut answer1 = 0;
        let mut answer2 = 0;
        for (y, x) in trailheads {
            let sq = &grid.get(&(y, x)).unwrap();
            let sq = *sq;
            let ends = sq.finals.len();
            // println!("({x}, {y}): {ends}");
            answer1 += ends;
            answer2 += sq.paths;
        }
        println!("{answer1}");
        println!("{answer2}");
        Ok(answer2)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
