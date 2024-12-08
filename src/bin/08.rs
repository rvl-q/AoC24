use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "08"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODOne: Solve Part 1 of the puzzle
        let mut lines= Vec::new();
        let mut ants = 0;
        let mut antennas: HashMap<char, Vec<(_,_)>> = HashMap::new();
        for (row, line) in reader.lines().enumerate() {
            let line = line?;
            for (col, c) in line.chars().enumerate(){
                if c == '.'{
                    continue;
                }
                ants += 1;
                let pos = antennas.get_mut(&c);
                match pos {
                    Some(pos) => pos.push((row, col)),
                    None => {
                        let mut pos = Vec::new();
                        pos.push((row, col));
                        antennas.insert(c, pos);
                    },
                }
            }
            lines.push(line);
        }
        let lines = lines;
        println!("Dim: {}, {}", lines.len(), lines[0].len());
        println!("Antennas: {ants}");
        fn anodes(a: (usize, usize), b: (usize, usize), m: usize) -> Vec<(usize, usize)> {
            let ay = a.0 as i32;
            let ax = a.1 as i32;
            let by = b.0 as i32;
            let bx = b.1 as i32;
            let dx = bx - ax;
            let dy = by - ay;
            let n0y = ay - dy;
            let n1y = by + dy;
            let n0x = ax - dx;
            let n1x = bx + dx;
            // println!("{a:?}, {b:?}, {m}");
            // println!("dbg: dy:{dy}, dx:{dx}");
            // println!("({n0y}, {n0x}),({n1y} {n1x})");
            fn ok(i: i32, mx: usize) -> bool{
                i >= 0 && i < mx as i32
            }
            let mut res = Vec::new();
            if ok(n0x, m) & ok(n0y, m){
                res.push((n0y as usize, n0x as usize));
            }
            if ok(n1x, m) & ok(n1y, m){
                res.push((n1y as usize, n1x as usize));
            }
            // println!("Res: {res:?}");
            res
        }
        let max = lines.len();
        let mut nodes: HashMap<(usize, usize), i32> = HashMap::new();
        for (_k, v) in antennas{
            // println!("{k}: {v:?}");
            for i in 0..v.len()-1{
                for j in i+1..v.len(){
                    //
                    let aa = anodes(v[i], v[j], max);
                    for a in aa{
                        nodes.insert(a,1);
                    }
                }
            }
            // anodes(v[0], v[1], max);
        }
        // println!("{nodes:?}");
        println!("Nodes: {}",nodes.len());
        let answer = nodes.len();
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines= Vec::new();
        // let mut ants = 0;
        let mut antennas: HashMap<char, Vec<(_,_)>> = HashMap::new();
        for (row, line) in reader.lines().enumerate() {
            let line = line?;
            for (col, c) in line.chars().enumerate(){
                if c == '.'{
                    continue;
                }
                // ants += 1;
                let pos = antennas.get_mut(&c);
                match pos {
                    Some(pos) => pos.push((row, col)),
                    None => {
                        let mut pos = Vec::new();
                        pos.push((row, col));
                        antennas.insert(c, pos);
                    },
                }
            }
            lines.push(line);
        }
        let lines = lines;
        // println!("Dim: {}, {}", lines.len(), lines[0].len());
        // println!("Antennas: {ants}");
        fn anodes(a: (usize, usize), b: (usize, usize), m: usize) -> Vec<(usize, usize)> {
            let ay = a.0 as i32;
            let ax = a.1 as i32;
            let by = b.0 as i32;
            let bx = b.1 as i32;
            let dx = bx - ax;
            let dy = by - ay;
            fn ok(i: i32, mx: usize) -> bool{
                i >= 0 && i < mx as i32
            }
            let mut res = Vec::new();
            for i in -1 * m as i32..1 + m as i32{
                let nx = ax + i * dx;
                let ny = ay + i * dy;
                if ok(nx, m) & ok(ny, m){
                    res.push((ny as usize, nx as usize));
                }
            }
            // println!("Res: {res:?}");
            res
        }
        let max = lines.len();
        let mut nodes: HashMap<(usize, usize), i32> = HashMap::new();
        for (_k, v) in antennas{
            // println!("{k}: {v:?}");
            for i in 0..v.len()-1{
                for j in i+1..v.len(){
                    //
                    let aa = anodes(v[i], v[j], max);
                    for a in aa{
                        nodes.insert(a,1);
                    }
                }
            }
            // anodes(v[0], v[1], max);
        }
        // println!("{nodes:?}");
        println!("Nodes: {}",nodes.len());
        let answer = nodes.len();
        Ok(answer)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
