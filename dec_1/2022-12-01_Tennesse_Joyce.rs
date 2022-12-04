use std::fs;

fn main() {
    // Read the input file to a string
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");

    // Loop through and count the total calories carried by
    // each elf.
    let mut calories_carried = Vec::<i32>::new();
    let mut calories_counter: i32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            // New
            calories_carried.push(calories_counter);
            calories_counter = 0;
            continue;
        }
        // Cast to an integer, and add to the current elf's calories.
        calories_counter += line.parse::<i32>().unwrap();
    }

    // Find the elf with the most calories.
    let max_calories: &i32 = calories_carried
        .iter()
        .max()
        .expect("There aren't any elves!");
    println!("The elf with the most calories has {max_calories}.",);

    // Sort the array to find the three elves with the most calories.
    calories_carried.sort();
    let top3_total_calories: i32 = calories_carried.iter().rev().take(3).sum();
    println!("The top 3 elves have {top3_total_calories} calories total.",);
}
