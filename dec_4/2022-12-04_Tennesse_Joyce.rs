use std::fs;

struct Assignment {
    // Represents what sections an elf was assigned to. It's always a
    // consecutive list of ints, so we just store the endpoints (inclusive).
    start: i32,
    end: i32
}

impl Assignment {
    fn fully_contains(&self, other: &Assignment) -> bool {
        // Check if another elf's assignment is fully covered by this
        // elf's assignment.
        (self.start <= other.start) & (self.end >= other.end)
    }

    fn overlaps_with(&self, other: &Assignment) -> bool {
        // Check if there's any overlap at all with another assignment.
        if self.start <= other.start {
            return self.end >= other.start
        }
        else {
            return other.end >= self.start
        }
    }
}

#[cfg(test)]
mod tests {
    // Test that the overlap-checking functions work correctly.
    use super::*;

    #[test]
    fn test_full_overlap() {
        let assignment0 = Assignment{start: 0, end: 4};
        let assignment1 = Assignment{start: 1, end: 2};
        assert!(assignment0.overlaps_with(&assignment1));
        assert!(assignment0.fully_contains(&assignment1));
    }

    #[test]
    fn test_partial_overlap() {
        let assignment0 = Assignment{start: 2, end: 4};
        let assignment1 = Assignment{start: 0, end: 3};
        assert!(assignment0.overlaps_with(&assignment1));
        assert!(!assignment0.fully_contains(&assignment1));
    }
    
    #[test]
    fn test_no_overlap() {
        let assignment0 = Assignment{start: 3, end: 4};
        let assignment1 = Assignment{start: 0, end: 2};
        assert!(!assignment0.overlaps_with(&assignment1));
        assert!(!assignment0.fully_contains(&assignment1));
    }
    
    #[test]
    fn test_boundary_overlap() {
        let assignment0 = Assignment{start: 2, end: 4};
        let assignment1 = Assignment{start: 0, end: 2};
        assert!(assignment0.overlaps_with(&assignment1));
        assert!(!assignment0.fully_contains(&assignment1));
    }
}

fn parse_assignments(line: &str) -> [Assignment; 2] {
    // Parse a line of input into the pair of elf assignments.
    let sections: Vec<i32>  = line
        .split(&['-', ','][..])
        .map(|x| x.parse().expect("Unable to parse int"))
        .collect()
        ;
    [
        Assignment{start: sections[0], end: sections[1]},
        Assignment{start: sections[2], end: sections[3]}
    ]
}

fn check_fully_contained(assignments: [Assignment; 2]) -> i32 {
    // Check if one of the assignments contains the other, and
    // return an integer 0 or 1 (so that we can total them up).
    (
        assignments[0].fully_contains(&assignments[1])
        | assignments[1].fully_contains(&assignments[0])
    ) as i32
}


fn check_overlap(assignments: [Assignment; 2]) -> i32 {
    // Check for overlap between the assignments, and return an
    // integer 0 or 1 (so that we can total them up).
    assignments[0].overlaps_with(&assignments[1]) as i32
}

fn main() {
    // Read the input file to a string
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");

    // Part 1
    let num_fully_contained_pairs: i32 = input.lines().map(parse_assignments).map(check_fully_contained).sum();
    println!("There are {num_fully_contained_pairs} pairs where one elf fully contains their partner's assignment.");
    
    // Part 2
    let num_overlapping_pairs: i32 = input.lines().map(parse_assignments).map(check_overlap).sum();
    println!("There are {num_overlapping_pairs} pairs with overlaping assignments.");
}
