use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "24"; // TODOne: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST0: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"; // Small test
const TEST: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"; // TODOne: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        // TODOne: Solve Part 1 of the puzzle
        let mut inputs = HashMap::new();
        let mut gates: Vec<Vec<_>> = Vec::new();
        let mut first = true;
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    if l.is_empty() {
                        first = false;
                        continue;
                    }
                    if first {
                        let _parts: Vec<&str> = l.split(": ").collect_vec();
                        // let left = _parts[0] as ???;
                        let str_of_c_c: String = _parts[0].chars().collect();
                        let val: u8 = _parts[1].parse()?;
                        // println!("{}: {}", &str_of_c_c, val);
                        inputs.insert(str_of_c_c, val);
                    } else {
                        let _parts: Vec<&str> = l.split(" -> ").collect_vec();
                        let res : String = _parts[1].chars().collect();
                        let _parts: Vec<&str> = _parts[0].split(" ").collect_vec();
                        let inp1: String = _parts[0].chars().collect();
                        let inp2: String = _parts[2].chars().collect();
                        let op: String = _parts[1].chars().collect();
                        let mut tmp: Vec<_> = Vec::new();
                        tmp.push(inp1);
                        tmp.push(inp2);
                        tmp.push(op);
                        tmp.push(res);
                        gates.push(tmp);
                    }
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }
        // println!("i & g: {}, {}",inputs.len(), gates.len());
        // println!("{inputs:?}");
        // println!("{gates:?}");

        let mut good_gates: Vec<Vec<_>> = Vec::new();
        let and = "AND";
        let or = "OR";
        let xor = "XOR";
        let mut tmp_gates: Vec<Vec<_>> = Vec::new();
        for g in &gates {
            if !inputs.contains_key(&g[0]) || !inputs.contains_key(&g[1]) {
                // println!("not yet...");
                tmp_gates.push(g.clone());
            } else {
                // println!("yes");
                let a = inputs.get(&g[0]).unwrap();
                let b = inputs.get(&g[1]).unwrap();
                good_gates.push(g.clone());
                if g[2].eq(and) {
                    // println!("andy");
                    if inputs.contains_key(&g[3]) {
                        println!("error!!");
                    } else {
                        inputs.insert(g[3].clone(), a & b);
                    }
                } else if g[2].eq(or) {
                    // println!("ory");
                    if inputs.contains_key(&g[3]) {
                        println!("error!!");
                    } else {
                        inputs.insert(g[3].clone(), a | b);
                    }
                } else if g[2].eq(xor) {
                    // println!("xory");
                    if inputs.contains_key(&g[3]) {
                        println!("error!!");
                    } else {
                        inputs.insert(g[3].clone(), a ^ b);
                    }
                }
            }
        }
        // println!("Good: {good_gates:?} {}", good_gates.len());
        // println!("Tmp: {tmp_gates:?} {}", tmp_gates.len());
        // println!("Inputs: {inputs:?} {}", inputs.len());

        while !tmp_gates.is_empty() {
            let g = tmp_gates.get(0).unwrap().clone();
            tmp_gates.remove(0);
            //
            if !inputs.contains_key(&g[0]) || !inputs.contains_key(&g[1]) {
                // println!("not yet...");
                tmp_gates.push(g.clone());
            } else {
                // println!("yes");
                let a = inputs.get(&g[0]).unwrap();
                let b = inputs.get(&g[1]).unwrap();
                good_gates.push(g.clone());
                if g[2].eq(and) {
                    // println!("andy");
                    if inputs.contains_key(&g[3]) {
                        println!("error!!");
                    } else {
                        inputs.insert(g[3].clone(), a & b);
                    }
                } else if g[2].eq(or) {
                    // println!("ory");
                    if inputs.contains_key(&g[3]) {
                        println!("error!!");
                    } else {
                        inputs.insert(g[3].clone(), a | b);
                    }
                } else if g[2].eq(xor) {
                    // println!("xory");
                    if inputs.contains_key(&g[3]) {
                        println!("error!!");
                    } else {
                        inputs.insert(g[3].clone(), a ^ b);
                    }
                }
            }
        }

        // println!("Good: {good_gates:?} {}", good_gates.len());
        // println!("Tmp: {tmp_gates:?} {}", tmp_gates.len());
        // println!("Inputs: {inputs:?} {}", inputs.len());

        let zzz: Vec<_> = vec![
            "z00", "z01", "z02", "z03", "z04", "z05", "z06", "z07", "z08", "z09",
            "z10", "z11", "z12", "z13", "z14", "z15", "z16", "z17", "z18", "z19",
            "z20", "z21", "z22", "z23", "z24", "z25", "z26", "z27", "z28", "z29",
            "z30", "z31", "z32", "z33", "z34", "z35", "z36", "z37", "z38", "z39",
            "z40", "z41", "z42", "z43", "z44", "z45", "z46", "z47", "z48", "z49",
        ];
        let mut res = 0;
        for (i, z) in zzz.iter().enumerate() {
            // println!("{z}");
            let v = inputs.get(*z);
            if v.is_none() {
                break;
            }
            res += (1u64 << i) * *v.unwrap() as u64;
        }
        println!("{res}");
        let answer = res;
        Ok(answer)
    }

    // TODOne: Set the expected answer for the test input
    assert_eq!(4, part1(BufReader::new(TEST0.as_bytes()))?);
    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // println!();
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        // let mut inputs: HashMap<String, u8> = HashMap::new();
        // let manual_swaps= vec!["plchldr", "plchldr", "plchldr", "plchldr", "plchldr", "plchldr", "plchldr", "plchldr"];
        let manual_swaps = vec!["hnd", "z09", "tdv", "z16", "bks", "z23", "nrn", "tjp"];
        let mut swaps = HashMap::new();
        for i in 0..4 {
            swaps.insert(manual_swaps[2*i].to_string(), manual_swaps[1+2*i].to_string());
            swaps.insert(manual_swaps[1+2*i].to_string(), manual_swaps[2*i].to_string());
        }
        // println!("{swaps:?}");
        let mut gates: Vec<Vec<_>> = Vec::new();
        let mut first = true;
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    if l.is_empty() {
                        first = false;
                        continue;
                    }
                    if first {
                        // skip this input and generate bit patterns later
                    } else {
                        let _parts: Vec<&str> = l.split(" -> ").collect_vec();
                        let res : String = _parts[1].chars().collect();
                        let _parts: Vec<&str> = _parts[0].split(" ").collect_vec();
                        let inp1: String = _parts[0].chars().collect();
                        let inp2: String = _parts[2].chars().collect();
                        let op: String = _parts[1].chars().collect();
                        let mut tmp: Vec<_> = Vec::new();
                        tmp.push(inp1);
                        tmp.push(inp2);
                        tmp.push(op);
                        if swaps.contains_key(&res) {
                            // println!("swapping: {res}<->{}", swaps[&res]);
                            tmp.push(swaps[&res].clone());
                        } else {
                            // println!("debug! {res}");
                            tmp.push(res);
                        }
                        gates.push(tmp);
                    }
                },
                Err(e) => {
                    println!("oh no: {}", e);
                }
            }
        }

        // bit test
        // all zeros was ok
        let xxx: Vec<_> = vec![
            "x00", "x01", "x02", "x03", "x04", "x05", "x06", "x07", "x08", "x09",
            "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19",
            "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29",
            "x30", "x31", "x32", "x33", "x34", "x35", "x36", "x37", "x38", "x39",
            "x40", "x41", "x42", "x43", "x44",
        ];
        let yyy: Vec<_> = vec![
            "y00", "y01", "y02", "y03", "y04", "y05", "y06", "y07", "y08", "y09",
            "y10", "y11", "y12", "y13", "y14", "y15", "y16", "y17", "y18", "y19",
            "y20", "y21", "y22", "y23", "y24", "y25", "y26", "y27", "y28", "y29",
            "y30", "y31", "y32", "y33", "y34", "y35", "y36", "y37", "y38", "y39",
            "y40", "y41", "y42", "y43", "y44",
        ];
        let mut okay = true;
        for i in 0..45 {
            for j in 1..4 {
                let mut inputs: HashMap<String, u8> = HashMap::new();

                for x in &xxx {
                    inputs.insert((*x).to_string(), 0u8);
                }
                for y in &yyy {
                    inputs.insert((*y).to_string(), 0u8);
                }
                if j == 1 {
                    inputs.insert(xxx[i].to_string(), 1u8);
                    // inputs.insert(yyy[i].to_string(), 1u8);
                }
                if j == 2 {
                    // inputs.insert(xxx[i].to_string(), 1u8);
                    inputs.insert(yyy[i].to_string(), 1u8);
                }
                if j == 3 {
                    inputs.insert(xxx[i].to_string(), 1u8);
                    inputs.insert(yyy[i].to_string(), 1u8);
                }

                let mut good_gates: Vec<Vec<_>> = Vec::new();
                let and = "AND";
                let or = "OR";
                let xor = "XOR";
                let mut tmp_gates: Vec<Vec<_>> = Vec::new();
                for g in &gates {
                    if !inputs.contains_key(&g[0]) || !inputs.contains_key(&g[1]) {
                        // println!("now...");
                        tmp_gates.push(g.clone());
                    } else {
                        let a = inputs.get(&g[0]).unwrap();
                        let b = inputs.get(&g[1]).unwrap();
                        good_gates.push(g.clone());
                        if g[2].eq(and) {
                            // println!("andy");
                            if inputs.contains_key(&g[3]) {
                                println!("error!!");
                            } else {
                                inputs.insert(g[3].clone(), a & b);
                            }
                        } else if g[2].eq(or) {
                            // println!("ory");
                            if inputs.contains_key(&g[3]) {
                                println!("error!!");
                            } else {
                                inputs.insert(g[3].clone(), a | b);
                            }
                        } else if g[2].eq(xor) {
                            // println!("xory");
                            if inputs.contains_key(&g[3]) {
                                println!("error!!");
                            } else {
                                inputs.insert(g[3].clone(), a ^ b);
                            }
                        }
                    }
                }

                while !tmp_gates.is_empty() {
                    let g = tmp_gates.get(0).unwrap().clone();
                    tmp_gates.remove(0);
                    //
                    if !inputs.contains_key(&g[0]) || !inputs.contains_key(&g[1]) {
                        // println!("not yet...");
                        tmp_gates.push(g.clone());
                    } else {
                        // println!("yes");
                        let a = inputs.get(&g[0]).unwrap();
                        let b = inputs.get(&g[1]).unwrap();
                        good_gates.push(g.clone());
                        if g[2].eq(and) {
                            // println!("andy");
                            if inputs.contains_key(&g[3]) {
                                println!("error!!");
                            } else {
                                inputs.insert(g[3].clone(), a & b);
                            }
                        } else if g[2].eq(or) {
                            // println!("ory");
                            if inputs.contains_key(&g[3]) {
                                println!("error!!");
                            } else {
                                inputs.insert(g[3].clone(), a | b);
                            }
                        } else if g[2].eq(xor) {
                            // println!("xory");
                            if inputs.contains_key(&g[3]) {
                                println!("error!!");
                            } else {
                                inputs.insert(g[3].clone(), a ^ b);
                            }
                        }
                    }
                }

                let zzz: Vec<_> = vec![
                    "z00", "z01", "z02", "z03", "z04", "z05", "z06", "z07", "z08", "z09",
                    "z10", "z11", "z12", "z13", "z14", "z15", "z16", "z17", "z18", "z19",
                    "z20", "z21", "z22", "z23", "z24", "z25", "z26", "z27", "z28", "z29",
                    "z30", "z31", "z32", "z33", "z34", "z35", "z36", "z37", "z38", "z39",
                    "z40", "z41", "z42", "z43", "z44", "z45", "z46", "z47", "z48", "z49",
                ];
                let mut res = 0;
                for (i, z) in zzz.iter().enumerate() {
                    // println!("{z}");
                    let v = inputs.get(*z);
                    if v.is_none() {
                        break;
                    }
                    res += (1u64 << i) * *v.unwrap() as u64;
                }
                if j == 3 {
                    if !(res == 2 << i) {
                        print!("Bad bits at: {} {} ", i, j);
                        println!("Should be: {} ...but is: {res}", 2u64<<i);
                        okay = false;
                    }
                } else {
                    if !(res == 1 << i) {
                        print!("Bad bits at: {} {} ", i, j);
                        println!("Should be: {} ...but is: {res}", 1u64<<i);
                        okay = false;
                    }
                }
            }
        }
        let mut answer = "".to_string();
        if okay {
            println!("All single bits OK!");
            let mut sorted_swaps = manual_swaps.clone();
            sorted_swaps.sort();
            sorted_swaps.iter().for_each(|s| answer = answer.clone().add(*s).add(","));
            answer.pop();
        }
        Ok(answer)
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
// Bad bits at: 8 3 Should be: 512 ...but is: 1024
// Bad bits at: 9 1 Should be: 512 ...but is: 1024
// Bad bits at: 9 2 Should be: 512 ...but is: 1024
// Bad bits at: 15 3 Should be: 65536 ...but is: 131072
// Bad bits at: 16 1 Should be: 65536 ...but is: 131072
// Bad bits at: 16 2 Should be: 65536 ...but is: 131072
// Bad bits at: 16 3 Should be: 131072 ...but is: 65536
// Bad bits at: 22 3 Should be: 8388608 ...but is: 16777216
// Bad bits at: 23 1 Should be: 8388608 ...but is: 16777216
// Bad bits at: 23 2 Should be: 8388608 ...but is: 16777216
// Bad bits at: 23 3 Should be: 16777216 ...but is: 8388608
// Bad bits at: 37 1 Should be: 137438953472 ...but is: 274877906944
// Bad bits at: 37 2 Should be: 137438953472 ...but is: 274877906944
// Bad bits at: 37 3 Should be: 274877906944 ...but is: 137438953472
//
// === Part 1 ===
// 4
// src\bin\24.rs:234 took 4.4566ms.
// Result = 53190357879014
//
// === Part 2 ===
// All single bits OK!
// src\bin\24.rs:450 took 515.8576ms.
// Result = bks,hnd,nrn,tdv,tjp,z09,z16,z23

