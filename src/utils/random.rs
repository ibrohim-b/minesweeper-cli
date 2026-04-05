use rand::{self, Rng};

pub fn random_range(min: usize, max: usize) -> usize {
  let mut rng = rand::thread_rng();
  rng.gen_range(min..max)
}