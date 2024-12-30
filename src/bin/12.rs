use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
// use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "12"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
AAAA
BBCD
BBCC
EEEC
"; // TODOne: Add the test input
const TEST1: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"; //
const TEST2: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"; //
const TEST3: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"; //
const TEST4: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"; //

const NORTH: (isize, isize) = (-1,0);
const EAST: (isize, isize) = (0,1);
const SOUTH: (isize, isize) = (1,0);
const WEST: (isize, isize) = (0,-1);

#[derive(Debug, Clone)]
struct Square {
    y: usize,
    x: usize,
    lbl: char,
    // region: (usize, usize);
    explored: bool,
    // valid: bool,
    fences: Vec<usize>,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODOne: Solve Part 1 of the puzzle
        let offsets: HashMap<usize, (isize, isize)> = HashMap::from([
            (0, NORTH), (1, EAST), (2, SOUTH), (3, WEST),
        ]);

        let grid: &mut Vec<Vec<char>> = &mut Vec::new();
        let garden_grid: &mut Vec<Vec<Square>> = &mut Vec::new();
        let mut all: HashSet<(usize, usize)> = HashSet::new();
        for (y, line) in reader.lines().enumerate() {
            // println!("{}", line?);
            let lin = line?;
            // let lie = lin.trim();
            let mut row = Vec::new();
            let mut g_row = Vec::new();
            // row.push('#');
            for (x, c) in lin.chars().enumerate() {
                // println!("{x}");
                // print!("{c}");
                // println!("{}", row.len,());
                row.push(c);
                all.insert((y, x));
                let sq = Square {
                    y,
                    x,
                    lbl: c,
                    explored: false,
                    // valid: true,
                    fences: Vec::new(),
                };
                g_row.push(sq);
            }
            // println!("{}", row.len());
            // row.push('#');
            // if y == 0 {
            //     let mut tmp = Vec::new();
            //     for _i in 0..row.len() {
            //         tmp.push('#');
            //     }
            //     grid.push(tmp);
            // }
            grid.push(row);
            garden_grid.push(g_row);
            // println!();
        }
        let xsz = grid[0].len();
        // let mut tmp = Vec::new();
        // for _i in 0..xsz {
        //     tmp.push('#');
        // }
        // grid.push(tmp);
        let ysz = grid.len();
        println!("{xsz},{ysz}");
        // println!("{xxx:?}");
        // for r in &*grid {
        //     println!("{r:?}");
        // }
        println!("Size: {}", all.len());

        // processing...
        let mut first = (0,0);
        let mut region = (0,0);
        let _dummy = first;
        let _dummy = region;
        // let mut all: HashSet<(usize, usize)> = HashSet::new();
        let mut current_region: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
        let mut regions = HashMap::new();
        let mut region_fences = HashMap::new();
        while !all.is_empty() {
            if current_region.is_empty() {
                first = all.iter().next().unwrap().clone();
                region = (first.0, first.1);
            } else {
                let now = current_region.iter().next().unwrap().clone();
                current_region.remove(&now);
                first = now.0;
                region = now.1;
            }
            // region = (first.0, first.1);
            all.remove(&first);
            // println!("Size: {}", all.len());
            // println!("{first:?}");
            let sq = garden_grid[first.0][first.1].clone();
            // println!("{sq:?}");
            let sq_lbl = sq.lbl;
            let mut tmp_fances = Vec::new();
            for d in 0..4usize {
                let dir = offsets[&d];
                let ny = first.0 as isize + dir.0;
                let nx = first.1 as isize + dir.1;
                if ny < 0 || ny >= ysz as isize {
                    tmp_fances.push(d);
                    continue;
                }
                if nx < 0 || nx >= xsz as isize {
                    tmp_fances.push(d);
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                let n_lbl = garden_grid[ny][nx].lbl;
                if sq_lbl == n_lbl {
                    if garden_grid[ny][nx].explored {
                        continue;
                    }
                    // println!("Yes: ({ny}, {nx}) @({}, {})", sq.y, sq.x);
                    current_region.insert(((ny, nx), region));
                } else {
                    tmp_fances.push(d);
                }
            }
            let sq = &mut garden_grid[sq.y][sq.x];
            sq.explored = true;
            sq.fences = tmp_fances;
            if regions.get(&region).is_none() {
                regions.insert(region, vec![(sq.y, sq.x)]);
                region_fences.insert(region, sq.fences.len());
            } else {
                let mut v: Vec<(usize, usize)> = (&regions.get(&region).unwrap()).to_vec();
                v.push((sq.y, sq.x));
                regions.insert(region, v);
                region_fences.insert(region, sq.fences.len() + region_fences.get(&region).unwrap());
            }
            // println!("{sq:?}");
        }

        // println!("{regions:?}, {}", regions.len());
        // println!("{:?}", region_fences);
        let mut res = 0;
        for (k, v) in regions {
            res += v.len() * region_fences.get(&k).unwrap();
        }
        println!("{res}");
        let answer = res;
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(140, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(1930, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let offsets: HashMap<usize, (isize, isize)> = HashMap::from([
            (0, NORTH), (1, EAST), (2, SOUTH), (3, WEST),
        ]);

        let grid: &mut Vec<Vec<char>> = &mut Vec::new();
        let garden_grid: &mut Vec<Vec<Square>> = &mut Vec::new();
        let mut all: HashSet<(usize, usize)> = HashSet::new();
        for (y, line) in reader.lines().enumerate() {
            let lin = line?;
            let mut row = Vec::new();
            let mut g_row = Vec::new();
            for (x, c) in lin.chars().enumerate() {
                row.push(c);
                all.insert((y, x));
                let sq = Square {
                    y,
                    x,
                    lbl: c,
                    explored: false,
                    fences: Vec::new(),
                };
                g_row.push(sq);
            }
            grid.push(row);
            garden_grid.push(g_row);
        }
        let xsz = grid[0].len();
        let ysz = grid.len();
        // println!("{xsz},{ysz}");
        // println!("Size: {}", all.len());

        // processing...
        let mut current; // = (0,0);
        let mut region; // = (0,0);
        let mut current_region: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
        let mut regions = HashMap::new();
        let mut region_fences = HashMap::new();
        let mut region_corners_test = HashMap::new();
        let mut region_corners= HashMap::new();
        let mut region_all_fences = HashMap::new();
        while !all.is_empty() {
            // println!("while... {}", all.len());
            if current_region.is_empty() {
                current = all.iter().next().unwrap().clone();
                region = (current.0, current.1);
            } else {
                let now = current_region.iter().next().unwrap().clone();
                current_region.remove(&now);
                current = now.0;
                region = now.1;
            }
            // println!("while... @region:{:?}", region);
            all.remove(&current);
            let sq = garden_grid[current.0][current.1].clone();
            let sq_lbl = sq.lbl;
            let mut tmp_fences = Vec::new();
            let mut fences_set = HashSet::new();
            for d in 0..4usize {
                let dir = offsets[&d];
                let ny = current.0 as isize + dir.0;
                let nx = current.1 as isize + dir.1;
                if ny < 0 || ny >= ysz as isize {
                    tmp_fences.push(d);
                    continue;
                }
                if nx < 0 || nx >= xsz as isize {
                    tmp_fences.push(d);
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                let n_lbl = garden_grid[ny][nx].lbl;
                if sq_lbl == n_lbl {
                    if garden_grid[ny][nx].explored {
                        continue;
                    }
                    current_region.insert(((ny, nx), region));
                } else {
                    tmp_fences.push(d);
                }
            }
            let sq = &mut garden_grid[sq.y][sq.x];
            sq.explored = true;
            sq.fences = tmp_fences.clone();
            if regions.get(&region).is_none() {
                regions.insert(region, vec![(sq.y, sq.x)]);
                region_fences.insert(region, sq.fences.len());
                let corners;
                if sq.fences.len() == 4 {
                    corners = 4usize;
                } else if sq.fences.len() == 3 {
                    corners = 2;
                } else if sq.fences.len() == 2 {
                    corners = sq.fences.iter().sum::<usize>() % 2;
                } else {
                    corners = 0;
                }
                region_corners_test.insert(region, corners);
                for d in tmp_fences.iter() {
                    fences_set.insert((current.0, current.1, d.to_owned()));
                }
                region_all_fences.insert(region, fences_set);
            } else {
                let mut v: Vec<(usize, usize)> = (&regions.get(&region).unwrap()).to_vec();
                v.push((sq.y, sq.x));
                regions.insert(region, v);
                region_fences.insert(region, sq.fences.len() + region_fences.get(&region).unwrap());
                let corners;
                if sq.fences.len() == 4 {
                    corners = 4usize;
                } else if sq.fences.len() == 3 {
                    corners = 2;
                } else if sq.fences.len() == 2 {
                    corners = sq.fences.iter().sum::<usize>() % 2;
                } else {
                    corners = 0;
                }
                region_corners_test.insert(region, corners + region_corners_test.get(&region).unwrap());
                //
                fences_set = (region_all_fences.get(&region).unwrap()).clone();
                for d in tmp_fences.iter() {
                    fences_set.insert((current.0, current.1, d.to_owned()));
                }
                region_all_fences.insert(region, fences_set);
            }
        }
        // println!("{:?}", region_all_fences);
        // println!("Debug, before for loop..\n");
        let mut _restarts = 0;
        for (k, _v) in &regions {
            region_corners.insert(k, 0usize);
            let region_segments: &mut HashSet<(usize, usize, usize)> = &mut region_all_fences.get(k).unwrap().clone();
            while !region_segments.is_empty() {
                let mut segment;
                segment = region_segments.iter().next().unwrap().clone();
                let first_segment = segment.clone();
                region_segments.remove(&segment);
                _restarts += 1;

                let mut d = segment.2;
                let mut y = segment.0;
                let mut x = segment.1;
                loop {
                    // println!("Loop start... {y},{x},{d} @{k:?} {}", garden_grid[y][x].lbl);
                    // println!("{region_segments:?}");
                    // clockwise (on the outside)
                    let cw = (d + 1) % 4;
                    // right
                    if region_segments.contains(&(y, x, cw)) {
                        // println!("Yes, Right: {y},{x},{cw} @{k:?}");
                        // region_set.remove(&(y, x, cw));
                        // cw = (cw + 1) % 4;
                        d = (d + 1) % 4;
                        region_corners.insert(k, 1 + region_corners.get(k).unwrap());
                    // straight
                    } else if region_segments.contains(&(
                            (y as isize + offsets[&((d + 1) % 4)].0) as usize,
                            (x as isize + offsets[&((d + 1) % 4)].1) as usize,
                            d)) {
                        // println!("Yes, Straight: {y},{x},{d} @{k:?}");
                        y = (y as isize + offsets[&((d + 1) % 4)].0) as usize;
                        x = (x as isize + offsets[&((d + 1) % 4)].1) as usize;
                        // region_set.remove(&(y, x, d));
                    // left
                    } else if region_segments.contains(
                        &((y as isize + offsets[&((d + 1) % 4)].0 + offsets[&d].0) as usize,
                          (x as isize + offsets[&((d + 1) % 4)].1 + offsets[&d].1) as usize,
                          (d + 3) % 4)
                    ) {
                        // println!("Yes, Left: {},{},{} @{k:?}",
                        //          (y as isize + offsets[&((d + 1) % 4)].0 + offsets[&d].0) as usize,
                        //          (x as isize + offsets[&((d + 1) % 4)].1 + offsets[&d].1) as usize, (d + 3) % 4);
                        y = (y as isize + offsets[&((d + 1) % 4)].0 + offsets[&d].0) as usize;
                        x = (x as isize + offsets[&((d + 1) % 4)].1 + offsets[&d].1) as usize;
                        d = (d + 3) % 4;
                        region_corners.insert(k, 1 + region_corners.get(k).unwrap());
                    } else {
                        // println!("Else... {y},{x},{d} @{k:?}");
                        if !(first_segment.eq(
                            &(
                                (y as isize + offsets[&((d + 1) % 4)].0) as usize,
                                (x as isize + offsets[&((d + 1) % 4)].1) as usize,
                                d)
                        )) {
                            // println!("plus one");
                            region_corners.insert(k, 1 + region_corners.get(k).unwrap());
                        }
                    }
                    if !region_segments.contains(&(y, x, d)) {
                        // println!("Break at {:?}, {}, {}, {}", k, y, x, d);
                        // println!("Set is: {region_segments:?}");
                        break;
                    }
                    segment = (y, x, d);
                    region_segments.remove(&segment);
                }
            }
        }
        // println!("{:?}", region_corners);
        // println!("{:?}", region_all_fences);
        // println!("{region_corners_test:?}");
        // let mut res = 0;
        // for (k, v) in &regions {
        //     res += v.len() * region_corners_test.get(k).unwrap();
        //     // this does not work, missing inside corners...
        //     println!("corners... {} x {} = {}",v.len(),
        //         region_corners_test.get(&k).unwrap(),
        //         v.len() * region_corners_test.get(&k).unwrap());
        // }
        // println!("Not all corners accounted for... {res}");

        let mut res = 0;
        for (k, v) in &regions {
            res += v.len() * region_corners.get(k).unwrap();
            // println!("area x sides = {} x {} = {}",v.len(),
            //     region_corners.get(&k).unwrap(),
            //     v.len() * region_corners.get(&k).unwrap());
        }
        println!("Counting corners in {} regions with {_restarts} restarts, which means {} \"hole(s)\". \nTotal cost: {res}"
                 , region_fences.len(), _restarts - region_fences.len());

        let answer = res;
        Ok(answer)
    }

    assert_eq!(80, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(436, part2(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(1206, part2(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(236, part2(BufReader::new(TEST3.as_bytes()))?);
    assert_eq!(368, part2(BufReader::new(TEST4.as_bytes()))?);
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
