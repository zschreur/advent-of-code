pub struct Puzzle;

struct OasisHistory()

impl Puzzle {
    fn new(input: &str) -> Self {
        Self
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {}

    fn run_part_two(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "";

    #[test]
    fn test() {}
}
