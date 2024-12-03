fn interpret_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let contents: Vec<Vec<u8>> = input.split("\n").map(|x| x.as_bytes().to_vec()).collect();

    let (list1, list2): (Vec<i32>, Vec<i32>) = contents
        .iter()
        .map(|x| x.iter().map(|x| *x as char).collect::<String>())
        .flat_map(|x| {
            x.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .unzip();

    return (list1, list2);
}

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let (list1, list2) = interpret_lists(input);
    return calculate_distance(list1, list2);
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let (list1, list2) = interpret_lists(input);
    return calculate_similarity(list1, list2);
}

fn calculate_distance(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    //sort each list in ascending order
    let mut list1 = list1.clone();
    let mut list2 = list2.clone();
    list1.sort();
    list2.sort();

    let mut distance: i32 = 0;

    for i in 0..list1.len() {
        let calculate = list1[i] - list2[i];
        // insure the distance is positive by taking the absolute value
        distance += calculate.abs();
    }
    return distance;
}

// This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.
fn calculate_similarity(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    //sort each list in ascending order
    let mut list1 = list1.clone();
    let mut list2 = list2.clone();
    list1.sort();
    list2.sort();

    let mut similarity: i32 = 0;

    for i in 0..list1.len() {
        let calculate = list1[i] * list2.iter().filter(|&x| *x == list1[i]).count() as i32;
        similarity += calculate;
    }
    return similarity;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let (list1, list2) = interpret_lists(
            "3 4
            4 3
            2 5
            1 3
            3 9
            3 3",
        );
        let distance = calculate_distance(list1, list2);
        assert_eq!(distance, 11);
    }

    #[test]
    fn test_calculate_similarity() {
        let (list1, list2) = interpret_lists(
            "3 4
            4 3
            2 5
            1 3
            3 9
            3 3",
        );
        let similarity = calculate_similarity(list1, list2);
        assert_eq!(similarity, 31);
    }
}
