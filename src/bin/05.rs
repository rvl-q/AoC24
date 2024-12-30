use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;
use std::collections::HashMap;

const DAY: &str = "05"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // TODOne: Solve Part 1 of the puzzle
        let mut rules: Vec<Vec<u8>> = Vec::new();
        let mut pages: Vec<Vec<u8>> = Vec::new();
        let mut first = true;
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    if l.is_empty() {
                        first = false;
                        continue;
                    }
                    if first {
                        let parts: Vec<&str> = l.split("|").collect_vec();
                        //println!("{parts:?}");
                        let mut row: Vec<u8> = Vec::new();
                        for p in parts {
                            let n: u8 = p.parse()?;
                            row.push(n);
                        }
                        rules.push(row);
                        //
                    } else {
                        // println!("{l}"); break;
                        let parts: Vec<&str> = l.split(",").collect_vec();
                        //println!("{parts:?}");
                        let mut row: Vec<u8> = Vec::new();
                        for p in parts {
                            let n: u8 = p.parse()?;
                            row.push(n);
                        }
                        //OK, no bad apples here!
                        // if !((row.len() as i32 + 1) % 2 == 0 ) {
                        //     print!("{:?} ", row);
                        // }
                        pages.push(row);
                    }
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }

        // println!("debug: {}, {}", rules.len(), pages.len());
        let mut the_rules: HashMap<u8, Vec<u8>> = HashMap::new();
        for rule in rules {
            if the_rules.contains_key( &rule[0]) {
                // println!("kukkuu! {}", rule[0]);
                let mut tmpvec = the_rules.get(&rule[0]).unwrap().to_owned();
                tmpvec.push(rule[1]);
                // println!("debug: {:?}", tmpvec);
                the_rules.insert(rule[0], tmpvec);
                //the_rules
            } else {
                let mut tmpvec = Vec::new();
                tmpvec.push(rule[1]);
                the_rules.insert(rule[0], tmpvec);
            }
        }

        // for (k,v) in &the_rules{
        //     println!("debug: {} -> {:?} {}", k, v, v.len());
        // }
        let mut answer = 0;

        for page in pages{
            let mut ok = true;
            for (i, p) in page.iter().enumerate(){
                // println!("{i}: {p} ...{}", page[0]);
                if i == page.len()-1{
                    continue;
                }
                let rule = the_rules.get(p);
                if rule.is_none(){
                    // println!("dbg: {:?}", page);
                    ok = false;
                    break;
                }
                let rule = rule.unwrap();
                for idx in i+1..page.len(){
                    // println!("{:?}", rule);
                    if rule.contains(&page[idx]){
                        // println!("yes...")
                    } else {
                        // println!("no! {}", &page[idx]);
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                // println!("dbg OK: {:?}", page);
                let n = page[(page.len()+1)/2-1] as i32;
                // println!("{n}");
                answer += n;
                // println!("{n}, {answer}");
            }
        }

        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    // assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut rules: Vec<Vec<u8>> = Vec::new();
        let mut pages: Vec<Vec<u8>> = Vec::new();
        let mut first = true;
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    if l.is_empty() {
                        first = false;
                        continue;
                    }
                    if first {
                        let parts: Vec<&str> = l.split("|").collect_vec();
                        //println!("{parts:?}");
                        let mut row: Vec<u8> = Vec::new();
                        for p in parts {
                            let n: u8 = p.parse()?;
                            row.push(n);
                        }
                        rules.push(row);
                        //
                    } else {
                        // println!("{l}"); break;
                        let parts: Vec<&str> = l.split(",").collect_vec();
                        //println!("{parts:?}");
                        let mut row: Vec<u8> = Vec::new();
                        for p in parts {
                            let n: u8 = p.parse()?;
                            row.push(n);
                        }
                        //OK, no bad apples here!
                        // if !((row.len() as i32 + 1) % 2 == 0 ) {
                        //     print!("{:?} ", row);
                        // }
                        pages.push(row);
                    }
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }

        // println!("debug: {}, {}", rules.len(), pages.len());
        let mut the_rules: HashMap<u8, Vec<u8>> = HashMap::new();
        for rule in rules {
            if the_rules.contains_key( &rule[0]) {
                // println!("kukkuu! {}", rule[0]);
                let mut tmpvec = the_rules.get(&rule[0]).unwrap().to_owned();
                tmpvec.push(rule[1].to_owned());
                // println!("debug: {:?}", tmpvec);
                the_rules.insert(rule[0], tmpvec.to_owned());
                //the_rules
                // println!("ruling...{:?}", tmpvec);
            } else {
                let mut tmpvec = Vec::new();
                tmpvec.push(rule[1].to_owned());
                the_rules.insert(rule[0], tmpvec.clone());
                // println!("ruling 0...{:?}", tmpvec);
            }

        }

        // for (k,v) in &the_rules{
        //     println!("debug: {} -> {:?} {}", k, v, v.len());
        // }

        let mut answer = 0;

        for page in pages{
            let mut ok = true;
            for (i, p) in page.iter().enumerate(){
                // println!("{i}: {p} ...{}", page[0]);
                if i == page.len()-1{
                    continue;
                }
                let rule = the_rules.get(p);
                if rule.is_none(){
                    // println!("dbg: {:?}", page);
                    ok = false;
                    break;
                }
                let rule = rule.unwrap();
                for idx in i+1..page.len(){
                    // println!("{:?}", rule);
                    if rule.contains(&page[idx]){
                        // println!("yes...")
                    } else {
                        // println!("no! {}", &page[idx]);
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                // println!("dbg OK: {:?}", page);
                let _n = page[(page.len()+1)/2-1] as i32;
                // println!("{n}");
                // answer += n;
                // println!("{n}, {answer}");
            } else {
                let mut temp =Vec::new();
                for pp in &page{
                    temp.push(pp.clone());
                }
                // println!("{:?}", temp);
                temp.sort_by(|a,b| {
                    let mut x = 0;
                    let mut y = 0;
                    if the_rules[a].contains(b){
                        // println!("{b} is in {a}: {:?}", the_rules[a]);
                        x = 1;
                    }
                    if the_rules[b].contains(a){
                        // println!("{a} is in {b}: {:?}", the_rules[b]);
                        y = 1;
                    }
                    y.cmp(&x)
                });
                // println!("{:?}", temp);
                let n = (temp.len()+1)/2-1;
                // println!("{n}");
                let n = &temp[n];
                // println!("{:?}",n);
                let n = *n as i32;
                // println!("{:?}",n);
                answer += n;
                // println!("{:?}", &page);
                // println!("{:?}", &temp);
                let _the_last_temp = &temp[temp.len()-2];
                // println!("{:?}", the_last_temp);
                // println!("{:?}", &temp);
                // println!("{}, {}",n, answer);
            }
            // if ok {
            //     // println!("dbg OK: {:?}", page);
            //     // let n = page[(page.len()+1)/2-1] as i32;
            //     // println!("{n}");
            //     // answer += n;
            // }
            // println!("dbg loop {:?}", page);
        }

        Ok(answer)
    }

    // assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);
    // assert_eq!(61, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
// 5588 part 1 OK.
//
// 5331
// OK!