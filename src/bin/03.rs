use regex::Regex;
use lazy_static::lazy_static;
use std::borrow::Cow;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fmt;
use adv_code_2024::*;

const DAY: &str = "03"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"; // TODOne: Add the test input
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\
"; // Different!!!

fn remove_parts(before: &str) -> Cow<str> {
    lazy_static! {
        static ref MY_REGEX : Regex = Regex::new(
            // r"(?ms)(?P<n>don't\(\))(?P<i>.*)(?P<d>do\(\))" // greedy - last possible end match
            r"(?ms)(?P<n>don't\(\))(?P<i>.*?)(?P<d>do\(\))" // non greedy - first match
            ).unwrap();
    }
    MY_REGEX.replace_all(before, "")
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODOne: Solve Part 1 of the puzzle
        struct RegTest<'a> {
            op1: &'a str,
            op2: &'a str,
            nop1: usize,
            nop2: usize,
        }
        impl<'a> fmt::Display for RegTest<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Product: {} x {} = {}", self.op1, self.op2, self.nop1 * self.nop2)
            }
        }
        let re = Regex::new(r#"(?x)    # 0?
            (mul\()     # 1
            (\d{1,3})   # op 1 @2
            (,)         # 3
            (\d{1,3})   # op 2 @4
            (\))        # 5
          "#,)?;

        // let input = &mut "".to_string();
        // reader.clone().read_to_string(input);

        let mut lines: Vec<_> = Vec::new();
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    lines.push(l);
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        println!("{}", lines.len());
        // println!("{}", &lines[0]);

        let mut res = 0;
        for line in lines {
            let foo = re.captures_iter(&line).filter_map(|cap| {
                let groups = (cap.get(2), cap.get(4));
                match groups {
                    (Some(op1), Some(op2)) => Some(RegTest {
                        op1: op1.as_str(),
                        op2: op2.as_str(),
                        nop1: op1.as_str().parse().unwrap(),
                        nop2: op2.as_str().parse().unwrap()
                    }, ),
                    _ => None,
                }
            });
            // let foo = foo.map(|m| m.to_string()).collect::<Vec<_>>();
            let foo = foo.map(|m| (m.nop1, m.nop2));
            // println!("{foo:?}");
            // println!("{}", foo.len());
            for f in foo {
                // println!("{:?}", f);
                res += f.0 * f.1;
            }
        }
        println!("{res}");
        let answer = res;
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        struct RegTest {
            nop1: usize,
            nop2: usize,
        }
        // impl<'a> fmt::Display for RegTest<'a> {
        //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //         write!(f, "Product: {} x {} = {}", self.op1, self.op2, self.nop1 * self.nop2)
        //     }
        // }
        let re = Regex::new(r#"(?x)    # 0?
            (mul\()     # 1
            (\d{1,3})   # op 1 @2
            (,)         # 3
            (\d{1,3})   # op 2 @4
            (\))        # 5
          "#,)?;

        let _re2 = Regex::new(r#"(?x)    # 0?
            (don't\(\))     # 1
            (.)*   # op 1 @2
            (do\(\))         # 3
            #(\d{1,3})   # op 2 @4
            #(\))        # 5
          "#,)?;

        let before = "2012-03-14, don't()2013-01-15do() and 2014-07-05";
        let after = remove_parts(before);
        assert_eq!(after, "2012-03-14,  and 2014-07-05");

        let input = &mut "".to_string();
        let size = &reader.read_to_string(input)?;
        println!("File size: {}", size);

        let mut res = 0;
        // println!("{input:?}");
        let input_filtered = remove_parts(input);
        // println!("{input_filtered:?}");
        let fmp = re.captures_iter(&input_filtered).filter_map(|cap| {
            let groups = (cap.get(2), cap.get(4));
            match groups {
                (Some(op1), Some(op2)) => Some(RegTest {
                    nop1: op1.as_str().parse().unwrap(),
                    nop2: op2.as_str().parse().unwrap()
                }, ),
                _ => None,
            }
        });
        let foo = fmp.map(|m| (m.nop1, m.nop2));
        // println!("{fmp:?}");
        // println!("{}", fmp.len());
        for f in foo {
            // println!("{:?}", f);
            res += f.0 * f.1;
        }
        println!("{res}");
        let answer = res;
        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
