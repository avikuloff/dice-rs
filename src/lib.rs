extern crate rand;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use rand::{Rng, SeedableRng};
use rand::distributions::Uniform;
use rand::rngs::SmallRng;
use std::num::NonZeroU32;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Die {
    faces: NonZeroU32,
}

impl Default for Die {
    fn default() -> Self {
        Die { faces: NonZeroU32::new(20).unwrap() }
    }
}

impl Die {
    /// Creates a new Die with the specified number of faces
    pub fn new(faces: NonZeroU32) -> Die {
        Die { faces }
    }

    /// Roll one die. Returns random number in the range from 1 to [`Die::faces()`].
    pub fn roll(&self) -> u32 {
        let mut rng = SmallRng::from_entropy();

        rng.sample(Uniform::new_inclusive(1, self.faces().get()))
    }

    /// Number of Die sides
    pub fn faces(&self) -> NonZeroU32 {
        self.faces
    }
}

/// Roll the Dice and return results of the Roll for each Die.
///
/// # Examples
///
/// Roll 3d6 `3 six-sided die` and get sum of the results
/// ```
/// # use roll_dice::roll;
/// # use std::num::NonZeroU32;
///
/// let rolled = roll(3, NonZeroU32::new(6).unwrap() );
/// let sum: u32 = rolled.iter().sum();
///
/// assert!(sum > 2 && sum < 19)
/// ```
pub fn roll(amount: u32, faces: NonZeroU32) -> Vec<u32> {
    let die = Die::new(faces);
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
    fn die_roll_d6() {
        let faces = NonZeroU32::new(6).unwrap();
        let d6 = Die::new(faces);

        for _ in 0..99 {
            let roll = d6.roll();
            assert!(roll > 0 && roll <= faces.get());
        }
    }

    #[test]
    fn die_roll_d1() {
        let d1 = Die::new(NonZeroU32::new(1).unwrap());

        assert_eq!(d1.roll(), 1);
    }

    #[test]
    fn die_faces() {
        let d6 = Die::new(NonZeroU32::new(6).unwrap());

        assert_eq!(d6.faces().get(), 6)
    }

    #[test]
    fn roll_vector_length_equals_number_of_dice() {
        let length = roll(3, NonZeroU32::new(6).unwrap()).len();

        assert_eq!(length, 3);
    }

    #[test]
    fn roll_vector_contains_numbers_from_valid_range() {
        let amount = 3;
        let faces = NonZeroU32::new(6).unwrap();

        for _ in 0..99 {
            let vec = roll(amount, faces);
            assert!(vec.iter().all(|v| v <= &faces.get() && v > &0));
        }
    }

    #[test]
    fn roll_zero_dice() {
        let amount = 0;
        let faces = NonZeroU32::new(6).unwrap();

        assert_eq!(roll(amount, faces).len(), 0)
    }
}
