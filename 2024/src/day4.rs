//she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

//This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

//MMMSXXMASM
//MSAMXMSMSA
//AMXSXMAAMM
//MSAMASMSMX
//XMASAMXAMM
//XXAMMXXAMA
//SMSMSASXSS
//SAXAMASAAA
//MAMMMXMMMM
//MXMXAXMASX
#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let found_words = find_words(&grid);
    assert_eq!(found_words, 2536);
    found_words
}

// The best data structure for this problem would be a 2D vector (Vec<Vec<char>>) to represent the grid.
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_words(grid: &Vec<Vec<char>>) -> usize {
    let mut found = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            found += check_word(grid, i, j, "XMAS");
        }
    }
    found
}

fn check_word(grid: &Vec<Vec<char>>, i: usize, j: usize, word: &str) -> usize {
    let mut found = 0;
    found += check_horizontal(grid, i, j, word);
    found += check_vertical(grid, i, j, word);
    found += check_diagonal(grid, i, j, word);
    found += check_diagonal_backwards(grid, i, j, word);
    found
}

fn check_horizontal(grid: &Vec<Vec<char>>, i: usize, j: usize, word: &str) -> usize {
    let mut found = 0;
    if j + word.len() <= grid[i].len() {
        let snippet: String = grid[i][j..j + word.len()].iter().collect();
        if snippet == word || snippet == word.chars().rev().collect::<String>() {
            found += 1;
        }
    }
    found
}

fn check_vertical(grid: &Vec<Vec<char>>, i: usize, j: usize, word: &str) -> usize {
    let mut found = 0;
    if i + word.len() <= grid.len() {
        let snippet: String = (0..word.len()).map(|k| grid[i + k][j]).collect();
        if snippet == word || snippet == word.chars().rev().collect::<String>() {
            found += 1;
        }
    }
    found
}

fn check_diagonal(grid: &Vec<Vec<char>>, i: usize, j: usize, word: &str) -> usize {
    let mut found = 0;
    if i + word.len() <= grid.len() && j + word.len() <= grid[i].len() {
        let snippet: String = (0..word.len()).map(|k| grid[i + k][j + k]).collect();
        if snippet == word || snippet == word.chars().rev().collect::<String>() {
            found += 1;
        }
    }
    found
}

fn check_diagonal_backwards(grid: &Vec<Vec<char>>, i: usize, j: usize, word: &str) -> usize {
    let mut found = 0;
    if i + word.len() <= grid.len() && j >= word.len() - 1 {
        let snippet: String = (0..word.len()).map(|k| grid[i + k][j - k]).collect();
        if snippet == word || snippet == word.chars().rev().collect::<String>() {
            found += 1;
        }
    }
    found
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_word() {
        let grid = parse_input(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );

        assert_eq!(find_words(&grid), 18);
    }
}
