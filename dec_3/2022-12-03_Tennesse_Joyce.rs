use std::fs;
use iterchunks::IterChunks;

fn get_priority(item: &u8) -> usize {
    // Get the priority of an item minus one (so that it's zero-indexed).
    // a..z are 0..25, and A..Z are 26..51
    if item > &b'Z' {
        //Lowercase
        (item - b'a').into()
    }
    else {
        //Uppercase
        (item - b'A' + 26).into()
    }
}

fn get_rucksack_counts(contents: &str) -> [i32; 52]{
    // Count how times each item appears in a rucksack.
    let mut counts = [0; 52];
    for item in contents.bytes(){
        counts[get_priority(&item)] += 1;
    }
    return counts
}

fn find_item_in_both_compartments(contents: &str) -> usize {
    // Find the duplicated item, and return its priority.
    let mut first_compartment = String::from(contents);
    let split_point = contents.len() / 2;
    let second_compartment = first_compartment.split_off(split_point);
    let first_compartment_counts = get_rucksack_counts(&first_compartment);
    let second_compartment_counts = get_rucksack_counts(&second_compartment);

    for idx in 0..52{
        if (first_compartment_counts[idx] > 0) & (second_compartment_counts[idx] > 0){
            return idx + 1
        }
    }
    panic!("No duplicates found in {contents}.")
}

fn find_badge(contents: [&str; 3]) -> usize{
    // Find the item that all three elves have in common.
    let elf0 = get_rucksack_counts(contents[0]);
    let elf1 = get_rucksack_counts(contents[1]);
    let elf2 = get_rucksack_counts(contents[2]);

    for idx in 0..52{
        if (elf0[idx] > 0) & (elf1[idx] > 0) & (elf2[idx] > 0){
            return idx + 1
        }
    }
    panic!("No badge found.")
}

fn main() {
    // Read the input file to a string
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");

    let misplaced_items_total_priority: usize = input.lines().map(find_item_in_both_compartments).sum();
    println!("The total priority of the misplaced items is {misplaced_items_total_priority}.");

    
    let badges_total_priority: usize = input.lines().array_chunks().map(find_badge).sum();
    println!("The total priority of the badges {badges_total_priority}.");
}
