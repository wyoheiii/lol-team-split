use rand::{rngs::StdRng, Rng, SeedableRng};
use crate::pipeline::TeamOptimizer;
use crate::domain::states::{AssignedTeams, Lobby};
use crate::domain::types::Player;
use crate::engine::evaluator::{Evaluator};


pub struct JointEnumeratingOptimizer {
  pub evaluator: Evaluator,
  pub rng: Option<StdRng>,

}
impl JointEnumeratingOptimizer {
  pub fn new(evaluator: Evaluator, seed: Option<u64>) -> Self {
    let rng = seed.map(StdRng::seed_from_u64);
    Self { evaluator, rng }
  }
}


impl TeamOptimizer for JointEnumeratingOptimizer {
  fn optimize(&self, lobby: &Lobby) -> AssignedTeams {
  let players = lobby.players();


  let mut best_val = f64::INFINITY;
  let mut cands: Vec<AssignedTeams> = Vec::new();


  for mask in 0u16..(1<<10) {
    if mask.count_ones() != 5 { continue; }
    let mut red = Vec::with_capacity(5); let mut blue = Vec::with_capacity(5);
  for i in 0..10 {
    if (mask>>i)&1 == 1 {
      red.push(players[i].clone());
    } else {
      blue.push(players[i].clone());
    }
  }
  let red: [Player;5] = red.try_into().unwrap();
  let blue: [Player;5] = blue.try_into().unwrap();


  // 各チームで「希望>サブ>オフ（オフは高MMR担当）」の最良割当を確定
  let (red_map, _kr, sr) = self.evaluator.best_assignment( &red);
  let (blue_map, _kb, sb) = self.evaluator.best_assignment(&blue);
  let diff = (sr - sb).abs();


  if diff + 1e-9 < best_val {
    best_val = diff; cands.clear();
    cands.push(AssignedTeams{ red: red_map, blue: blue_map });
  }
  else if (diff - best_val).abs() <= 1e-9 {
    cands.push(AssignedTeams{ red: red_map, blue: blue_map });
  }
}


  if cands.is_empty() {
    panic!("no candidate split");
  }

  if let Some(mut rng) = self.rng.clone() {
    let idx = rng.random_range(0..cands.len());
    return cands[idx].clone();
  }

  cands[0].clone()
}
}