use std::fs;
use regex::Regex;

struct Instruction {
    // A single step in the rearrangement procedure.
    source: usize,
    destination: usize,
    number: usize
}

impl Instruction {
    fn from_capture(cap: regex::Captures) -> Self{
        Instruction {
            source: cap[2].parse::<usize>().unwrap() - 1,
            destination: cap[3].parse::<usize>().unwrap() - 1,
            number: cap[1].parse().unwrap()
        }
    }
}

enum CraneVersion{
    CrateMover9000,
    CrateMover9001
}


struct GiantCargoCrane {
    stacks: Vec<Vec<char>>,
    version: CraneVersion
}

impl GiantCargoCrane {
    fn add_crate(&mut self, pos: usize, name: char){
        self.stacks[pos].push(name);
    }

    fn pop_crate(&mut self, pos: usize) -> char {
        self.stacks[pos].pop().expect("Stack {pos} is empty!")
    }

    fn new(num_stacks: usize, version: CraneVersion) -> Self {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        for _ in 0..num_stacks {
            stacks.push(Vec::new());
        }
        GiantCargoCrane {
            stacks: stacks,
            version: version
        }
    }

    fn num_stacks(&self) -> usize {
        self.stacks.len()
    }

    fn move_crates(&mut self, instruction: &Instruction){
        match self.version {
            CraneVersion::CrateMover9000 => self.move_crates_9000(instruction),
            CraneVersion::CrateMover9001 => self.move_crates_9001(instruction),
        }
    }

    fn move_crates_9000(&mut self, instruction: &Instruction){
        for _ in 0..instruction.number{
            let cr = self.pop_crate(instruction.source);
            self.add_crate(instruction.destination, cr);
        }
    }

    fn move_crates_9001(&mut self, instruction: &Instruction){
        let mut crane_stack: Vec<char> = Vec::new();
        for _ in 0..instruction.number{
            let cr = self.pop_crate(instruction.source);
            crane_stack.push(cr);
        }
        for _ in 0..instruction.number{
            let cr = crane_stack.pop().expect("Crane isn't holding any crates.");
            self.add_crate(instruction.destination, cr);
        }
    }

    fn get_list_of_top_crates(&self) -> Vec<&char>{
        self
            .stacks
            .iter()
            .map(|stack| stack.last().expect("Stack is empty"))
            .collect()
    }

    fn add_crates_from_input(&mut self, input: &str){
        // Parse the arrangement of crates from the input file, and create
        // a new "GiantCargoCrane" instance in that configuration.
    
        // First parse the arrangement of crates into a flat list, with
        // unoccupied positions encoded as "None".
        let crates_regex:Regex = Regex::new(r"\[[A-Z]\]|    ").unwrap();
        let crate_names: Vec<Option<char>> =
            crates_regex
            .captures_iter(&input)
            .map(parse_crate)
            .collect()
        ;
    
        // Add the crates to the stacks in reverse order, so that the first
        // ones we add end up at the bottom. Use modular arithmetic to find
        // which stack each crate should be added to.
        for (i, name) in crate_names.iter().enumerate().rev(){
            match name {
                Some(c) => self.add_crate(i % self.num_stacks(), *c),
                None => {}
            }
        }
    }
}


#[cfg(test)]
mod tests {
    // Test the behavior of the two different crane versions.
    use super::*;

    #[test]
    fn test_two_crates_9000() {
        let mut crane = GiantCargoCrane::new(2, CraneVersion::CrateMover9000);
        crane.add_crate(0, 'A');
        crane.add_crate(0, 'B');

        crane.move_crates(&Instruction{source: 0, destination: 1, number:2});
        assert_eq!(crane.stacks[1][1], 'A');
    }

    #[test]
    fn test_two_crates_9001() {
        let mut crane = GiantCargoCrane::new(2, CraneVersion::CrateMover9001);
        crane.add_crate(0, 'A');
        crane.add_crate(0, 'B');

        crane.move_crates(&Instruction{source: 0, destination: 1, number:2});
        assert_eq!(crane.stacks[1][1], 'B');
    }
    
}

fn parse_crate(cap: regex::Captures) -> Option<char>{
    let name = &cap[0];
    if name == "    " {
        None
    }
    else {
        Some(name.chars().nth(1).expect("Failed to parse crate {name}"))
    }
}



fn parse_instructions(input: &str) -> Vec<Instruction>{
    // Parse the instructions
    let instructions_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    instructions_regex
        .captures_iter(&input)
        .map(Instruction::from_capture)
        .collect()
}


fn main() {
    // Read the input file to a string
    let input: String = fs::read_to_string("input.txt").expect("Input file not found.");

    // A simple way to get the number of crates is to look at the length
    // of the first line. Each crate takes up 4 characters, but the last
    // crate only takes up 3.
    let num_stacks = input.lines().next().expect("Empty input").len() / 4 + 1;
    println!("Crates are arranged in {num_stacks} stacks.");

    // Initialize a crane object of each version with that many stacks.
    let mut crane1 = GiantCargoCrane::new(
        num_stacks, CraneVersion::CrateMover9000);
    let mut crane2 = GiantCargoCrane::new(
        num_stacks, CraneVersion::CrateMover9001);

    // Set up the initial configuration of crates for both cranes.
    crane1.add_crates_from_input(&input);
    crane2.add_crates_from_input(&input);

    // Parse the instructions
    let instructions = parse_instructions(&input);

    // Feed the list of instructions to both cranes.
    for i in instructions.iter(){
        crane1.move_crates(i);
        crane2.move_crates(i);
    }

    // Read out the final arrangements.
    print!("Final arrangement for CrateMover9000 (Part 1): ");
    for name in crane1.get_list_of_top_crates().iter(){
        print!("{name}");
    }
    println!("");
    print!("Final arrangement for CrateMover9001 (Part 2): ");
    for name in crane2.get_list_of_top_crates().iter(){
        print!("{name}");
    }

}
