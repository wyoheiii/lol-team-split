use crate::domain::types::{Player, Role, RoleMap};
use crate::param::eval::{EvalContext, TeamScore, penalty_multiplier_from_z};


#[derive(Debug, Clone, Copy)]
struct RoleEval {
  effw: f64,              // 重み付き有効MMR
  off_delta: usize,       // オフロールなら 1
  sub_delta: usize,       // サブロールなら 1
  off_mmr_neg_delta: f64, // off_mmr_neg_sum への加算分 (-base or 0.0)
}

#[derive(Clone)]
pub struct Evaluator { pub cfg: EvalContext }

impl Evaluator {
  pub fn new(cfg: EvalContext) -> Self {
    Self { cfg }
  }

  /// 希望>サブ>オフ（オフは高MMR担当）で最良割当とスコア
  pub fn best_assignment(&self, team: &[Player;5]) -> (RoleMap<Player>, PrefKey, f64) {
    let (pick, key, score) = self.best_perm_pref_first(team);
    let map = RoleMap {
      top: team[pick[0]].clone(),
      jg: team[pick[1]].clone(),
      mid: team[pick[2]].clone(),
      adc: team[pick[3]].clone(),
      sup: team[pick[4]].clone()
    };
    (map, key, score)
  }


  // /// 分割評価用（割当は返さずスコアだけ）
  // pub fn best_score(&self, team: &[Player;5]) -> (PrefKey, f64) {
  //   let (_pick, key, score) = self.best_perm_pref_first(team);
  //   (key, score)
  // }

  fn score(&self, p: &Player, role: Role) -> RoleEval {
      let base = self.cfg.eval.mmr.calculate(&p.rank);
      let is_main = role == p.main_role;
      let is_sub = !is_main && p.sub_role.contains(&role);

      let mut off_delta = 0usize;
      let mut sub_delta = 0usize;
      let mut off_mmr_neg_delta = 0f64;

      if !is_main && !is_sub {
          off_delta = 1;
          off_mmr_neg_delta = -base;
      }
      if is_sub {
          sub_delta = 1;
      }

      let z = self.cfg.lobby.z_from(base);
      let pen = self.cfg.penalty.total_penalty(p, role)
          as f64
          * penalty_multiplier_from_z(z, self.cfg.eval.flex_bias_alpha);

      let eff = base - pen;
      let effw = eff * self.cfg.role_weight.weight(&role);

      RoleEval {
          effw,
          off_delta,
          sub_delta,
          off_mmr_neg_delta,
      }
  }

  // pub fn score_assigned(&self, assigned: &RoleMap<Player>) -> (PrefKey, f64) {
  //   let pairs: [(Role, &Player); 5] = [
  //     (Role::Top, &assigned.top),
  //     (Role::Jg,  &assigned.jg),
  //     (Role::Mid, &assigned.mid),
  //     (Role::Adc, &assigned.adc),
  //     (Role::Sup, &assigned.sup),
  //   ];

  //   let mut off = 0usize;
  //   let mut sub = 0usize;
  //   let mut off_mmr_neg_sum = 0f64;
  //   let mut effw = [0f64; 5];

  //   for (i, (role, p)) in pairs.iter().enumerate() {
  //     let r = self.score(p, *role);
  //     effw[i] = r.effw;
  //     off += r.off_delta;
  //     sub += r.sub_delta;
  //     off_mmr_neg_sum += r.off_mmr_neg_delta;
  //   }

  //   let score = match self.cfg.eval.score {
  //     // TeamScore::Softmax { tau } => softmax_score(&effw, tau),
  //     TeamScore::TopK { k } => topk_score(&effw, k),
  //   };

  //   (
  //     PrefKey {
  //       off_count: off,
  //       off_mmr_neg_sum,
  //       sub_count: sub,
  //     },
  //     score,
  //   )
  // }

  fn best_perm_pref_first(&self, team: &[Player; 5]) -> ([usize; 5], PrefKey, f64) {
    let roles_arr = Role::ALL;
    let idxs = [0usize, 1, 2, 3, 4];
    let mut best_pick = [0usize; 5];
    let mut best_key = PrefKey {
      off_count: usize::MAX,
      off_mmr_neg_sum: f64::INFINITY,
      sub_count: usize::MAX,
    };
    let mut best_score = f64::NEG_INFINITY;

    for perm in permutations(idxs) {
      let mut off = 0usize;
      let mut sub = 0usize;
      let mut off_mmr_neg_sum = 0f64;
      let mut effw = [0f64; 5];

      for (ri, &role) in roles_arr.iter().enumerate() {
        let p = &team[perm[ri]];
        let r = self.score(p, role);
        effw[ri] = r.effw;
        off += r.off_delta;
        sub += r.sub_delta;
        off_mmr_neg_sum += r.off_mmr_neg_delta;
      }

      let score = match self.cfg.eval.score {
        // TeamScore::Softmax { tau } => softmax_score(&effw, tau),
        TeamScore::TopK { k } => topk_score(&effw, k),
      };

      let key = PrefKey {
          off_count: off,
          off_mmr_neg_sum,
          sub_count: sub,
      };

      let better =
        key.off_count < best_key.off_count ||
        (key.off_count == best_key.off_count
          && key.off_mmr_neg_sum.total_cmp(&best_key.off_mmr_neg_sum).is_lt()) ||
        (key.off_count == best_key.off_count
          && key.off_mmr_neg_sum.total_cmp(&best_key.off_mmr_neg_sum).is_eq()
          && key.sub_count < best_key.sub_count) ||
        (key.off_count == best_key.off_count
          && key.off_mmr_neg_sum.total_cmp(&best_key.off_mmr_neg_sum).is_eq()
          && key.sub_count == best_key.sub_count
          && score > best_score);

      if better {
        best_pick = perm;
        best_key = key;
        best_score = score;
      }
    }
    (best_pick, best_key, best_score)
  }
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PrefKey {
  pub off_count: usize,
  pub off_mmr_neg_sum: f64,
  pub sub_count: usize
}

// fn softmax_score(e:&[f64;5], tau:f64)->f64 {
//   let t = tau.max(1.0);
//   let m = e.iter().cloned().fold(f64::NEG_INFINITY,f64::max);
//   let exps:Vec<f64> = e.iter().map(|x|((x - m) / t).exp()).collect();
//   let den: f64 = exps.iter().sum();
//   e.iter().zip(&exps).map(|(x,w)| x * w).sum::<f64>() / den
// }

fn topk_score(e:&[f64;5], k:usize)->f64 {
  let mut v = e.to_vec();
  v.sort_by(|a,b| b.total_cmp(a));
  v.into_iter().take(k.min(5)).sum()
}

fn permutations(mut arr:[usize;5])->Vec<[usize;5]> {
  let mut v=Vec::new();
  heap_perm(5,&mut arr,&mut v);
  v
}

fn heap_perm(k:usize, arr:&mut [usize;5], out:&mut Vec<[usize;5]>) {
  if k==1{ out.push(*arr); return; }
  heap_perm(k-1,arr,out);
  for i in 0..k-1{
    if k%2==0{ arr.swap(i,k-1);}
    else {arr.swap(0,k-1);}
    heap_perm(k-1,arr,out);
  }
}