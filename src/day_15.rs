fn hash_step(s: &str) -> u128 {
    s.chars().fold(0, |acc, x| ((acc + x as u128) * 17) % 256)
}

fn run_initialization_sequence(seq: Vec<&str>) -> u128 {
    seq.iter().fold(0, |acc, x| acc + hash_step(x))
}

fn calculate_focusing_power(seq: Vec<&str>) -> u128 {
    let mut boxes: [Vec<(&str, u128)>; 256] = std::array::from_fn(|_| Vec::new());
    seq.iter().for_each(|step| {
        if let Some((label, focal_length)) = step.find("=").map(|i| step.split_at(i)) {
            let b = &mut boxes[hash_step(label) as usize];
            let focal_length = focal_length
                .split_at(1)
                .1
                .parse::<u128>()
                .expect("Unable to parse focal length");
            if let Some(entry) = b.iter_mut().find(|e| e.0 == label) {
                entry.1 = focal_length;
            } else {
                b.push((label, focal_length));
            }
        } else if let Some(label) = step.strip_suffix("-") {
            let b = &mut boxes[hash_step(label) as usize];
            if let Some((index, _)) = b.iter_mut().enumerate().find(|(_, e)| e.0 == label) {
                b.remove(index);
            }
        }
    });

    boxes.iter().enumerate().fold(0u128, |acc, (i, b)| {
        acc + (i as u128 + 1)
            * b.iter()
                .enumerate()
                .map(|(slot_number, (_, focal_length))| focal_length * (slot_number as u128 + 1))
                .sum::<u128>()
    })
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
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let res = run_initialization_sequence(self.0.split(",").collect());
        Ok(super::AOCResult::ULong(res))
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let res = calculate_focusing_power(self.0.split(",").collect());
        Ok(super::AOCResult::ULong(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_sample_part_one() {
        assert_eq!(
            run_initialization_sequence(INPUT.split(",").collect()),
            1320
        );
    }

    #[test]
    fn test_sample_part_two() {
        assert_eq!(calculate_focusing_power(INPUT.split(",").collect()), 145);
    }
}
