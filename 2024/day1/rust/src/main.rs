use std::io::BufRead;

fn main() {
    let (list1, list2) = interpret_lists("../input1");
    let distance = calculate_distance(list1, list2);
    println!("Distance: {}", distance);

    let (list1, list2) = interpret_lists("../input1");
    let similarity = calculate_similarity(list1, list2);
    println!("Similarity: {}", similarity);
}

fn interpret_lists(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = std::fs::File::open(filename).expect("File not found");
    // turn every line into a byte array
    let contents: Vec<Vec<u8>> = std::io::BufReader::new(contents)
        .lines()
        .map(|x| x.unwrap().into_bytes())
        .collect();

    // File looks like this
    // 69214   60950
    // 83241   49638

    // split the file into two lists
    // remove white spaces and convert to i32

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
        let (list1, list2) = interpret_lists("../input1.example");
        let distance = calculate_distance(list1, list2);
        assert_eq!(distance, 11);
    }

    #[test]
    fn test_calculate_similarity() {
        let (list1, list2) = interpret_lists("../input1.example");
        let similarity = calculate_similarity(list1, list2);
        assert_eq!(similarity, 31);
    }
}
