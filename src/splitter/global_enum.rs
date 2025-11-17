use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{domain::{states::{Lobby, SplitTeams}, Player, Role}, param::eval::{penalty_multiplier_from_z, EvalContext, LobbyStats, TeamScore}, pipeline::TeamSplitter};

pub struct GlobalEnumeratingSplitter {
  eval: EvalContext,
  rng : Option<StdRng>,
}

impl GlobalEnumeratingSplitter {
  pub fn new(seed: u64, eval: EvalContext) -> Self {
    Self { eval, rng: Some(StdRng::seed_from_u64(seed)) }
  }

  fn best_score_for_team(&self, team: &[Player;5]) -> f64 {
    let roles = Role::All.clone();
    let idxs = [0usize,1,2,3,4];
    let mut best = f64::NEG_INFINITY;
    for perm in permutations(idxs) {
    let mut effw = [0f64;5];
    for (ri, &role) in roles.iter().enumerate() {
      let p = &team[perm[ri]];
      effw[ri] = self.effective_weighted_for_eval(p, role);
    }
    let score = match self.eval.eval.score {
      TeamScore::Softmax { tau } => softmax_score(&effw, tau),
      TeamScore::TopK { k } => topk_score(&effw, k)
    };
    if score > best { best = score; }
    }
    best
  }


  fn effective_weighted_for_eval(&self, p: &Player, role: Role) -> f64 {
    let base = self.eval.eval.mmr.calculate(&p.rank);
    let z = self.eval.lobby.z_from(base);
    let pen = self.eval.penalty.total_penalty(p, role.clone()) as f64 * penalty_multiplier_from_z(z, self.eval.eval.flex_bias_alpha);
    let eff = base - pen;
    eff * self.eval.role_weight.weight(&role)
  }
}



fn softmax_score(effw: &[f64;5], tau: f64) -> f64 {
  let tau = tau.max(1.0);
  let maxv = effw.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
  let exps: Vec<f64> = effw.iter().map(|&x| ((x - maxv)/tau).exp()).collect();
  let denom: f64 = exps.iter().sum();
  effw.iter().zip(exps.iter()).map(|(x,e)| x*e).sum::<f64>() / denom
}

fn topk_score(effw: &[f64;5], k: usize) -> f64 {
  let mut v = effw.to_vec();
  v.sort_by(|a,b| b.total_cmp(a));
  v.iter().take(k.min(5)).sum()
}


fn permutations(mut arr: [usize;5]) -> Vec<[usize;5]> {
  let mut v = Vec::new();
  heap_permutation(5, &mut arr, &mut v);
  v
}

fn heap_permutation(k: usize, arr: &mut [usize;5], out: &mut Vec<[usize;5]>) {
  if k == 1 {
    out.push(*arr); return;
  }
  heap_permutation(k-1, arr, out);
  for i in 0..k-1 {
    if k % 2 == 0 { arr.swap(i, k-1); } else { arr.swap(0, k-1); }
    heap_permutation(k-1, arr, out);
  }
}

impl TeamSplitter for GlobalEnumeratingSplitter {
  fn split(&self, lobby: &Lobby) -> SplitTeams {
    let players = lobby.players().clone();

    let mut best_diff = f64::INFINITY;
    let mut cands = vec![];

    for mask in 0u16..(1<<10) {
      if mask.count_ones() != 5 {
        continue;
      }

      let mut red = Vec::with_capacity(5);
      let mut blue = Vec::with_capacity(5);

      for i in 0..10 {
        if (mask >> i) & 1 == 1 {
          red.push(players[i].clone());
        } else {
          blue.push(players[i].clone());
        }
      }

      let red: [Player; 5] = red.try_into().unwrap();
      let blue: [Player; 5] = blue.try_into().unwrap();
      let sr = self.best_score_for_team(&red);
      let sb = self.best_score_for_team(&blue);

      let diff = (sr - sb).abs();

      if diff + 1e-9 < best_diff {
        best_diff = diff;
        cands.clear();
        cands.push(SplitTeams::new(red, blue));
      } else if (diff - best_diff).abs() < 1e-9 {
        cands.push(SplitTeams::new(red, blue));
      }
    }
    if cands.is_empty() {
      panic!("No valid split found");
    }
    if let Some(mut rng) = self.rng.clone() {
      let idx = rng.random_range(0..cands.len());
      return cands[idx].clone();
    }
    cands[0].clone()
  }
}
