use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let report = read_lines("../input1");
    println!("Result: {}", analyze_report(report));

    let report = read_lines("../input1");
    println!("Result: {}", biased_anazlye_report(report));
}

// Input: need to split by line
// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9

fn read_lines(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open(filename).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| {
            l.expect("Could not parse line")
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
// No sorting is required.
fn analyze_report(report: Vec<Vec<i32>>) -> i32 {
    report.iter().map(|l| analyze_level(l)).sum()
}

fn biased_anazlye_report(report: Vec<Vec<i32>>) -> i32 {
    report
        .iter()
        .map(|l| {
            // if the level is 1, then it is safe otherwise cycle through removing one element to try and make it safe. if that fails, then it is unsafe
            if analyze_level(l) == 1 {
                return 1;
            }
            for i in 0..l.len() {
                let mut new_level = l.clone();
                new_level.remove(i);
                if analyze_level(&new_level) == 1 {
                    return 1;
                }
            }
            return 0;
        })
        .sum()
}

fn analyze_level(level: &Vec<i32>) -> i32 {
    let mut increase = false;
    let mut decrease = false;
    let mut great_diff = false;
    let mut same = false;

    for i in 0..level.len() - 1 {
        let numbers = (level[i], level[i + 1]);
        let diff = numbers.1 - numbers.0;
        if diff >= 1 {
            increase = true;
        }
        if diff <= -1 {
            decrease = true;
        }
        if diff > 3 || diff < -3 {
            great_diff = true;
        }
        if diff == 0 {
            same = true;
        }
    }
    if increase && !decrease && !great_diff && !same {
        // println!(
        //     "increase: {}, decrease: {}, great_diff {}, same {} : Safe",
        //     increase, decrease, great_diff, same
        // );
        return 1;
    } else if !increase && decrease && !great_diff && !same {
        // println!(
        //     "increase: {}, decease: {}, great_diff {}, same {} : Safe",
        //     increase, decrease, great_diff, same
        // );
        return 1;
    }
    // println!(
    //     "increase: {}, decease: {}, great_diff {}, same {} : Unsafe",
    //     increase, decrease, great_diff, same
    // );
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        assert_eq!(
            read_lines("../input1.example"),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }

    #[test]
    fn test_analyze_report() {
        assert_eq!(analyze_report(read_lines("../input1.example")), 2);
    }

    #[test]
    fn test_biased_analyze_report() {
        assert_eq!(biased_anazlye_report(read_lines("../input1.example")), 4);
    }
}
