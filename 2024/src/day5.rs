use std::error::Error;

#[derive(Debug)]
struct PrecedenceRule {
    number: u32,
    must_precede: u32,
}

struct ProblemData {
    rules: Vec<PrecedenceRule>,
    data: Vec<Vec<u32>>,
}

#[aoc(day5, part1)]
fn day5(input: &str) -> u32 {
    let data = split_data(input);
    let output = data
        .data
        .iter()
        .map(|x| check_order(x, &data.rules))
        .filter_map(|x| x.ok())
        .collect::<Vec<Vec<u32>>>();

    output.iter().map(|x| return_middle(x)).sum()
}

fn return_middle(x: &[u32]) -> u32 {
    x[x.len() / 2]
}

fn generate_precedence_rules(input: &Vec<&str>) -> Vec<PrecedenceRule> {
    input
        .iter()
        .map(|x| {
            let split: Vec<&str> = x.split("|").collect();
            PrecedenceRule {
                number: split[0].parse::<u32>().unwrap(),
                must_precede: split[1].parse::<u32>().unwrap(),
            }
        })
        .collect()
}

fn split_data(input: &str) -> ProblemData {
    // there is a blank line between the two groups
    let split_input: Vec<&str> = input.split("\n\n").collect();

    let precedence_rules =
        generate_precedence_rules(&split_input[0].lines().collect::<Vec<&str>>());

    let data: Vec<Vec<u32>> = split_input[1]
        .split("\n")
        .map(|x| x.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();

    ProblemData {
        rules: precedence_rules,
        data,
    }
}

fn check_order(data: &Vec<u32>, rules: &Vec<PrecedenceRule>) -> Result<Vec<u32>, Box<dyn Error>> {
    for i in 0..data.len() {
        let rules_for_value = rules.iter().filter(|x| x.number == data[i]);
        for rule in rules_for_value {
            if let Some(pos) = data.iter().position(|&x| x == rule.must_precede) {
                if pos < i {
                    // Print a great error message
                    return Err(format!(
                        "Value {} must come before value {}",
                        data[i], rule.must_precede
                    )
                    .into());
                }
            }
        }
    }
    Ok(data.clone())
}

fn order_data(data: &Vec<u32>, rules: &Vec<PrecedenceRule>) -> Result<Vec<u32>, Box<dyn Error>> {
    // if check_order passes return error
    if check_order(data, rules).is_ok() {
        return Err("Data is already ordered".into());
    }

    let mut ordered_data: Vec<u32> = data.clone();

    for value in data.len()..0 {
        let rules_for_value = rules
            .iter()
            .filter(|x| x.number == data[value])
            .filter(|x| {
                ordered_data
                    .iter()
                    .position(|&y| y == x.must_precede)
                    .unwrap()
                    > value
            });

        dbg!(&rules_for_value);
    }

    Err(format!("Data: {:?}", ordered_data).into())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1() {
        let input = "47|53
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
97,13,75,29,47";

        let data = split_data(input);

        // convert the output to a vector of vectors
        // don't include the values that errored
        let output: Vec<Vec<u32>> = data
            .data
            .iter()
            .map(|x| match check_order(x, &data.rules) {
                Ok(result) => Some(result),
                Err(e) => {
                    println!("Error in line {:?}: {}", x, e);
                    None
                }
            })
            .filter_map(|x| x)
            .collect();

        let middle = output
            .iter()
            .map(|x| return_middle(x))
            .collect::<Vec<u32>>();

        assert_eq!(middle, vec![61, 53, 29]);
        assert_eq!(143, middle.iter().sum::<u32>());
    }

    #[test]
    fn part2() {
        let input = "47|53
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
97,13,75,29,47";

        let data = split_data(input);

        // convert the output to a vector of vectors
        // don't include the values that errored
        let output: Vec<Vec<u32>> = data
            .data
            .iter()
            .map(|x| match order_data(x, &data.rules) {
                Ok(result) => Some(result),
                Err(e) => {
                    println!("Error in line {:?}: {}", x, e);
                    None
                }
            })
            .filter_map(|x| x)
            .collect();

        let middle = output
            .iter()
            .map(|x| return_middle(x))
            .collect::<Vec<u32>>();

        assert_eq!(middle, vec![47, 29, 47]);
        assert_eq!(123, middle.iter().sum::<u32>());
    }
}
