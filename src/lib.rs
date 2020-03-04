extern crate rand;

use rand::{Rng, SeedableRng};
use rand::distributions::Uniform;
use rand::rngs::SmallRng;

#[derive(Debug, Clone)]
pub struct Die(u32);

impl Default for Die {
    fn default() -> Self {
        Die(20)
    }
}

impl Die {
    /// Roll a Die. Returns a random number in the range of `sides`.
    pub fn roll(&self) -> u32 {
        let mut rng = SmallRng::from_entropy();

        rng.sample(Uniform::new_inclusive(1, self.0))
    }

    /// Number of faces
    pub fn num_faces(&self) -> u32 {
        self.0
    }
}

/// Roll the Dice and return results of the Roll for each Die.
pub fn roll(amount: u32, faces: u32) -> Vec<u32> {
    let die = Die(faces);
    let mut results = Vec::with_capacity(amount as usize);

    for _ in 0..amount {
        results.push(die.roll());
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn die_roll() {
        let faces = 6;
        let d6 = Die(faces);

        for _ in 0..99 {
            let roll = d6.roll();
            assert!(roll > 0 && roll <= faces);
        }
    }

    #[test]
    fn roll_vector_length_equals_faces() {
        let amount = 3;
        let faces = 6;

        let length = roll(amount, faces).len();

        assert_eq!(length, amount as usize);
    }

    #[test]
    fn roll_vector_contains_numbers_from_valid_range() {
        let amount = 3;
        let faces = 6;

        for _ in 0..99 {
            let vec = roll(amount, faces);
            assert!(vec.iter().all(|v| v <= &faces && v > &0));
        }
    }
}
