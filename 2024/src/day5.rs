use std::collections::HashMap;
use std::error::Error;

struct ProblemData {
    hash_map: HashMap<u32, u32>,
    data: Vec<Vec<u32>>,
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let data = split_data(input);

    // convert the output to a vector of vectors
    // don't include the values that errored
    let output: Vec<Vec<u32>> = data
        .data
        .iter()
        .map(|x| check_order(x, &data.hash_map))
        .filter_map(Result::ok)
        .collect();

    // get the middle value of each row
    let middle = output
        .iter()
        .map(|x| return_middle(x))
        .collect::<Vec<u32>>();

    // sum the middle values
    middle.iter().sum::<u32>() as usize
}
// Return the middle value of a sorted list
fn return_middle(data: &Vec<u32>) -> u32 {
    data[data.len() / 2]
}

// Premise of this program is that the first section is a hash of numbers.
// e.g 97|13
// 97 must come before 13
//
// I am thinking a hash map would be a good solution for this
fn hash_function(input: &Vec<&str>) -> HashMap<u32, u32> {
    // for every row in the input, split the row by the pipe character
    // and insert the first number as the key and the second number as the value

    let mut hash_map = HashMap::new();
    for row in input {
        let split = row.split("|").collect::<Vec<&str>>();
        let key = split[0].parse::<u32>().expect("Failed to parse key");
        let value = split[1].parse::<u32>().expect("Failed to parse value");
        hash_map.insert(key, value);
    }
    hash_map
}

fn split_data(input: &str) -> ProblemData {
    // there is a blank line between the two groups
    let split_input: Vec<&str> = input.split("\n\n").collect();

    let hashed_input = hash_function(&split_input[0].lines().collect::<Vec<&str>>());

    let data: Vec<Vec<u32>> = split_input[1]
        .split("\n")
        .map(|x| x.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();

    ProblemData {
        hash_map: hashed_input,
        data,
    }
}

fn check_order(data: &Vec<u32>, hash_map: &HashMap<u32, u32>) -> Result<Vec<u32>, Box<dyn Error>> {
    for i in 0..data.len() {
        if let Some(&value) = hash_map.get(&data[i]) {
            if let Some(pos) = data.iter().position(|&x| x == value) {
                if pos < i {
                    // Print a great error message
                    return Err(
                        format!("Value {} must come before value {}", data[i], value).into(),
                    );
                }
            }
        }
    }
    Ok(data.clone())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sort_data() {
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
            .map(|x| match check_order(x, &data.hash_map) {
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
}
