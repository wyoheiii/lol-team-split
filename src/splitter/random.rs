use crate::{domain::Player, pipeline::states::{Lobby, SplitTeams}, splitter::TeamSplitter};
use rand::{self, seq::SliceRandom, SeedableRng};
pub struct RandomSplitter {seed: u64}

impl RandomSplitter {
  pub fn new(seed: u64) -> Self {
    Self { seed }
  }
}

impl TeamSplitter for RandomSplitter {
  fn split(&self, lobby: &Lobby) -> SplitTeams {
    let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
    let mut idx = (0..10).collect::<Vec<usize>>();
    idx.shuffle(&mut rng);
    let a:[Player; 5] = idx[..5].iter().map(|&i| lobby.players()[i].clone()).collect::<Vec<Player>>().try_into().unwrap();
    let b:[Player; 5] = idx[5..].iter().map(|&i| lobby.players()[i].clone()).collect::<Vec<Player>>().try_into().unwrap();

    SplitTeams::new(a, b)
  }
}