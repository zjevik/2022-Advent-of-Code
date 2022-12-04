use std::collections::HashMap;
use std::fs;

fn get_total_score(input: &str, rubric: HashMap<&str, i32>) -> i32 {
    // Loop through and count the total calories carried by
    // each elf.
    let mut total_score: i32 = 0;
    for line in input.lines() {
        total_score += rubric[line];
    }
    total_score
}

fn main() {
    // Read the input file to a string
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");

    // Define what your score is for each outcome. There are two
    // contributions, first is from win/lose/draw, second is from
    // the shape you selected.
    let part1_rubric = HashMap::from([
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 0 + 3),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 0 + 2),
        ("C Z", 3 + 3),
    ]);

    let part2_rubric = HashMap::from([
        ("A X", 0 + 3),
        ("A Y", 3 + 1),
        ("A Z", 6 + 2),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 0 + 2),
        ("C Y", 3 + 3),
        ("C Z", 6 + 1),
    ]);

    let part1_total_score = get_total_score(&input, part1_rubric);
    let part2_total_score = get_total_score(&input, part2_rubric);

    println!("The total score for Part 1 is {part1_total_score}.");
    println!("The total score for Part 2 is {part2_total_score}.");
}
