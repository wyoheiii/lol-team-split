use std::f64;

use crate::{domain::{states::{AssignedTeams, SplitTeams}, types::RoleMap, Player, Role}, param::{eval::{penalty_multiplier_from_z, EvalContext}, penalty_matrix::PenaltyMatrix}, pipeline::RoleAssigner};

pub struct BruteForceAssigner {
  eval_context:EvalContext,
}

impl RoleAssigner for BruteForceAssigner {
  fn assign(&self, teams: SplitTeams) -> AssignedTeams {
    AssignedTeams {
      red: self.best_for_team(teams.red()),
      blue: self.best_for_team(teams.blue()),
    }
  }

}

impl BruteForceAssigner {
  pub fn new(eval_context:EvalContext) -> Self {
    Self { eval_context }
  }

  fn best_for_team(&self, team:&[Player;5]) -> RoleMap<Player> {
    let roles = Role::All;
    let idxs = [0usize,1,2,3,4];
    let mut best_score = f64::NEG_INFINITY;
    let mut best_pick = [0usize; 5];

    for perm in permutations(idxs) {
      let mut score = 0.0;

      for (ri, role) in roles.iter().enumerate() {
        let p = &team[perm[ri]];
        let base = self.eval_context.eval.mmr.calculate(&p.rank);
        let z_score = self.eval_context.lobby.z_from(base);

        let penalty = self.eval_context.penalty.total_penalty(p, role.clone())
          as f64 * penalty_multiplier_from_z(z_score, self.eval_context.eval.flex_bias_alpha);

        let eff = base - penalty;
        score += self.softmax_weight(eff, team, &perm) * eff * self.eval_context.role_weight.weight(role);
    }

    if score > best_score {
        best_score = score;
        best_pick = perm;
      }
    }

    RoleMap {
      top: team[best_pick[0]].clone(),
      jg: team[best_pick[1]].clone(),
      mid: team[best_pick[2]].clone(),
      adc: team[best_pick[3]].clone(),
      sup: team[best_pick[4]].clone(),
    }

  }

  fn softmax_weight(&self, target_eff:f64, team:&[Player;5], perm: &[usize; 5]) -> f64 {
  let tau = self.eval_context.eval.softmax_tau.max(1.0);
  let mut effs = [0f64; 5];

  for (ri, role) in Role::All.iter().enumerate() {
    let p = &team[perm[ri]];
    let base = self.eval_context.eval.mmr.calculate(&p.rank);
    let z_score = self.eval_context.lobby.z_from(base);
    let penalty = self.eval_context.penalty.total_penalty(p, role.clone())
      as f64 * penalty_multiplier_from_z(z_score, self.eval_context.eval.flex_bias_alpha);

    effs[ri] = base - penalty;

  }
  let maxv = effs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
  let exps = effs.iter().map(|&v| ((v - maxv) / tau).exp()).collect::<Vec<f64>>();
  let denom = exps.iter().sum::<f64>();


  ((target_eff - maxv) / tau).exp() / denom
}
}



// Heap's algorithmで全順列を生成
fn permutations(mut arr: [usize;5]) -> Vec<[usize;5]> {
  let mut v = Vec::new();
  heap_permutation(5, &mut arr, &mut v);
  v
}


fn heap_permutation(k: usize, arr: &mut [usize;5], v: &mut Vec<[usize;5]>) {
  if k == 1 { v.push(*arr); return; }
  heap_permutation(k-1, arr, v);

  for i in 0..k-1 {
    if k % 2 == 0 {
      arr.swap(i, k-1);
    } else {
      arr.swap(0, k-1);
    }
    heap_permutation(k-1, arr, v);
  }
}