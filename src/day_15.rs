fn hash_step(s: &str) -> u128 {
    s.chars().fold(0, |acc, x| ((acc + x as u128) * 17) % 256)
}

fn run_initialization_sequence(seq: Vec<&str>) -> u128 {
    seq.iter().fold(0, |acc, x| acc + hash_step(x))
}

pub struct Puzzle(String);

impl Puzzle {
    fn new(input: &str) -> Self {
        let seq = input.replace("\n", "");
        Self(seq)
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        let res = run_initialization_sequence(self.0.split(",").collect());
        println!("Part 1: {}", res);
    }

    fn run_part_two(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_sample() {
        assert_eq!(
            run_initialization_sequence(INPUT.split(",").collect()),
            1320
        );
    }
}
