use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "07"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"; // TODOne: Add the test input

struct Calib {
    res: i64,
    row: Vec<i64>,
    // good: bool,
}

fn try_one(seq: &mut Vec<i64>, nxt: i64, lim: i64) -> (){
    // println!("{:?}", &seq);
    // let mut prod: Vec<i64> = seq.clone().into_iter().map(|x| x * nxt).collect();
    let prod: Vec<i64> = seq.clone().into_iter().map(|x| x * nxt).collect();
    seq.iter_mut().for_each(|e| *e += nxt);
    // println!("{:?}", &seq);
    // println!("B: {:?}", prod);
    let mut prod: Vec<i64> = prod.clone().into_iter().filter(|e| *e <= lim).collect();
    // println!("F: {:?}", prode);
    seq.append(&mut prod);
    // seq.append(&mut prode);
    // println!("{:?}", &seq);
}

fn check_cal(kand: Calib) -> bool{
    // // test test
    // if [3267i64, 190, 292].contains(&kand.res){
    //     return true;
    // }
    let mut pos:Vec<i64> = Vec::new();
    pos.push(kand.row[0]);
    for k in kand.row[1..].iter(){
        try_one(&mut pos, *k, kand.res);
    }
    println!("{}: {:?} -> {}",kand.res, pos, pos.contains(&kand.res));
    pos.contains(&kand.res)
    // false
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        // TODOne: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut big: i64 = 0;
        let mut long = 0;
        for line in reader.lines() {
            let line = line?;
            // println!("{}", &line);
            let parts: Vec<_> = line.split(": ").collect();
            // println!("{:?}", &parts);
            let res: i64 = parts[0].parse()?;
            // println!("{res}");
            if res > big{
                big = res;
            }
            let mut row: Vec<i64> = Vec::new();
            let parts: Vec<_> = parts[1].split(" ").collect();
            // println!("{:?}", &parts);
            for p in parts{
                row.push(p.parse()?);
            }
            println!("{res} {:?}", &row);
            if row.len() > long{
                long = row.len();
            }
            let calib: Calib = Calib{
                res,
                row,
            };
            if check_cal(calib){
                answer += res;
            }

        }
        println!("{big} {long}");

        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

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
// Result = 1260333054159 OK!
//