use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODOne: Solve Part 1 of the puzzle
        let grid: &mut Vec<Vec<char>> = &mut Vec::new();
        let mut xxx: Vec<(usize, usize)> = Vec::new();
        for (y, line) in reader.lines().enumerate() {
            // println!("{}", line?);
            let lin = line?;
            // let lie = lin.trim();
            let mut row = Vec::new();
            for (x, c) in lin.chars().enumerate() {
                // println!("{x}");
                // print!("{c}");
                // println!("{}", row.len,());
                if c == 'X' {
                    row.push('X');
                    xxx.push((y,x))
                } else if c == 'M' {
                    row.push('M');
                } else if c == 'A' {
                    row.push('A');
                } else if c == 'S' {
                    row.push('S');
                } else {
                    println!("ELSE!");
                }
            }
            // println!("{}", row.len());
            grid.push(row);
            // println!();
        }
        let ysz = grid.len();
        let xsz = grid[0].len();
        println!("{xsz},{ysz}");
        // println!("{xxx:?}");
        let xmas = ['X', 'M', 'A', 'S'].to_vec();
        let mut dirs: Vec<(isize,isize)> = Vec::new();
        dirs.push((-1,0));
        dirs.push((-1,1));
        dirs.push((0,1));
        dirs.push((1,1));
        dirs.push((1,0));
        dirs.push((1,-1));
        dirs.push((0,-1));
        dirs.push((-1,-1));

        let mut answer = 0;
        for x_coord  in xxx {
            for dir in &dirs {
                let (y, x) = x_coord;
                for i in 0..4isize{
                    let xms_c = xmas[i as usize];
                    let dy = i * dir.0;
                    let dx = i * dir.1;
                    let ny = dy + y as isize;
                    let nx = dx + x as isize;
                    if ny < 0 || ny > ysz as isize -1 {
                        break;
                    }
                    if nx < 0 || nx > xsz as isize -1 {
                        break;
                    }
                    let ny = ny as usize;
                    let nx = nx as usize;
                    let mc = grid[ny][nx];
                    if mc != xms_c {
                        break;
                    }
                    // print!("{ny:?},");
                    // print!("{nx:?}:");
                    // print!("{mc} ");
                    if i == 3 {
                        answer += 1;
                    }
                }
            }
        }

        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid: &mut Vec<Vec<char>> = &mut Vec::new();
        let mut aaa: Vec<(usize, usize)> = Vec::new();
        for (y, line) in reader.lines().enumerate() {
            // println!("{}", line?);
            let lin = line?;
            // let lie = lin.trim();
            let mut row = Vec::new();
            for (x, c) in lin.chars().enumerate() {
                // println!("{x}");
                // print!("{c}");
                // println!("{}", row.len,());
                if c == 'X' {
                    row.push('X');
                } else if c == 'M' {
                    row.push('M');
                } else if c == 'A' {
                    row.push('A');
                    aaa.push((y, x));
                } else if c == 'S' {
                    row.push('S');
                } else {
                    println!("ELSE!");
                }
            }
            // println!("{}", row.len());
            grid.push(row);
            // println!();
        }
        let ysz = grid.len();
        let xsz = grid[0].len();
        println!("{xsz},{ysz}");
        // println!("{xxx:?}");
        let xmas = ['M', 'A', 'S'].to_vec();
        let mut dirs: Vec<(isize,isize)> = Vec::new();
        dirs.push((-1,1));
        dirs.push((1,1));
        dirs.push((1,-1));
        dirs.push((-1,-1));

        let mut answer = 0;
        for x_coord  in aaa {
            let mut a_cnt = 0;
            for dir in &dirs {
                let (y, x) = x_coord;
                for i in -1..2isize{
                    let xms_c = xmas[(i+1) as usize];
                    let dy = i * dir.0;
                    let dx = i * dir.1;
                    let ny = dy + y as isize;
                    let nx = dx + x as isize;
                    if ny < 0 || ny > ysz as isize -1 {
                        break;
                    }
                    if nx < 0 || nx > xsz as isize -1 {
                        break;
                    }
                    let ny = ny as usize;
                    let nx = nx as usize;
                    let mc = grid[ny][nx];
                    if mc != xms_c {
                        break;
                    }
                    // print!("{ny:?},");
                    // print!("{nx:?}:");
                    // print!("{mc} ");
                    if i == 1 {
                        a_cnt += 1;
                    }
                }
            }
            if a_cnt == 2 {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
