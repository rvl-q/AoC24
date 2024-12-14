use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "14"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const Y_SZ: isize = 103;
const X_SZ: isize = 101;
// const Y_SZ: isize = 7;
// const X_SZ: isize = 11;
const P1_STEPS: usize = 100;
const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"; // TODOne: Add the test input

#[derive(Debug, Clone)]
struct Robo {
    y: usize,
    x: usize,
    dy: isize,
    dx: isize,
}
impl Robo {
    fn step1(&mut self) -> () {
        self.y = ((Y_SZ + self.y as isize + self.dy) % Y_SZ) as usize;
        self.x = ((X_SZ + self.x as isize + self.dx) % X_SZ) as usize;
    }
    fn quad(&self) -> usize {
        if self.y == ((Y_SZ - 1) / 2) as usize {
            return 0
        }
        if self.x == ((X_SZ - 1) / 2) as usize {
            return 0
        }
        let mut q = 1;
        if self.x > ((X_SZ - 1) / 2) as usize {
            q += 1;
        }
        if self.y > ((Y_SZ - 1) / 2) as usize {
            q += 2;
        }
        q
    }
}
// try out "to string"
impl std::fmt::Display for Robo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "p:(y:{}, x:{})", self.y, self.x)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODOne: Solve Part 1 of the puzzle
        let mut robos: Vec<Robo> = Vec::new();
        let mut quads: Vec<usize> = vec![0,0,0,0];
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    let parts: Vec<&str> = l.split(" ").collect();
                    // println!("{parts:?}");
                    let y: usize = parts[0].split(",").collect::<Vec<_>>()[1].parse()?;
                    let dy: isize = parts[1].split(",").collect::<Vec<_>>()[1].parse()?;
                    let x: usize = parts[0].split(",").collect::<Vec<_>>()[0]
                        .split("=").collect::<Vec<_>>()[1].parse()?;
                    let dx: isize = parts[1].split(",").collect::<Vec<_>>()[0]
                        .split("=").collect::<Vec<_>>()[1].parse()?;
                    // println!("p: {y:?},{x}");
                    // println!("v: {dy:?},{dx}");
                    let mut robo = Robo {
                        x,y,dx,dy,
                    };
                    // println!("{robo} in Q{}", robo.quad());
                    // println!("{robo:?}");
                    for _ in 0..P1_STEPS {
                        let _ = &robo.step1();
                    }
                    // println!("{robo:?}");
                    // let mut quads: Vec<usize> = vec![0,0,0,0];
                    if robo.quad() > 0 {
                        // println!("{robo} in Q{}", robo.quad());
                        quads[robo.quad()-1] += 1;
                        // println!("{quads:?}")
                    }
                    robos.push(robo);
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        let answer = quads[0]*quads[1]*quads[2]*quads[3];
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    if X_SZ < 100 {
        assert_eq!(12, part1(BufReader::new(TEST.as_bytes()))?);
    } else {
        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        let result = time_snippet!(part1(input_file)?);
        println!("Result = {}", result);
    }
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<isize> {
        let mut robos: Vec<Robo> = Vec::new();
        // let mut quads: Vec<usize> = vec![0,0,0,0];
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    let parts: Vec<&str> = l.split(" ").collect();
                    let y: usize = parts[0].split(",").collect::<Vec<_>>()[1].parse()?;
                    let dy: isize = parts[1].split(",").collect::<Vec<_>>()[1].parse()?;
                    let x: usize = parts[0].split(",").collect::<Vec<_>>()[0]
                        .split("=").collect::<Vec<_>>()[1].parse()?;
                    let dx: isize = parts[1].split(",").collect::<Vec<_>>()[0]
                        .split("=").collect::<Vec<_>>()[1].parse()?;
                    let robo = Robo {
                        x,y,dx,dy,
                    };
                    robos.push(robo);
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        let mut grid: Vec<Vec<char>> = Vec::new();
        let row: Vec<char> = vec![' '; X_SZ as usize];
        for _ in 0..Y_SZ {
            grid.push(row.clone());
        }
        // println!("{grid:?}");
        let mut q_min = i32::MAX;
        let mut idx = 0;
        for _i in 0..Y_SZ * X_SZ {
            let mut quads = vec![0, 0, 0, 0];
            let r_old = robos.clone();
            for r in &mut robos {
                if r.quad() > 0 {
                    quads[r.quad()-1] += 1;
                }
                grid[r.y][r.x] = '#';
                r.step1();
            }
            let q =quads[0]*quads[1]*quads[2]*quads[3];
            if q < q_min {
                println!("{_i}, {q}");
                q_min = q;
                idx = _i;
                // print the grid
                // for r in &grid {
                //     for c in r {
                //         print!("{c}");
                //     }
                //     println!();
                // }
            }
            for r in r_old {
                grid[r.y][r.x] = ' ';
            }
        }
        println!();
        let answer = idx;
        Ok(answer)
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
