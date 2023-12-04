pub struct Puzzle;

impl Puzzle {
    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self)
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        println!("Part 1: {}", "NOT IMPLEMENTED");
    }
    fn run_part_two(&self) {
        println!("Part 2: {}", "NOT IMPLEMENTED");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {}
}
