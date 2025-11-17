use std::collections::HashSet;

use rand::Rng;

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
   pub fn best_assignment<R: Rng + ?Sized>(
    &self,
    team: &[Player; 5],
    rng: Option<&mut R>,
    priority_main: &HashSet<u32>,
  ) -> (RoleMap<Player>, PrefKey, f64) {
    let (pick, key, score) =
      self.best_perm_pref_first(team, rng, priority_main);

    let map = RoleMap {
      top: team[pick[0]].clone(),
      jg:  team[pick[1]].clone(),
      mid: team[pick[2]].clone(),
      adc: team[pick[3]].clone(),
      sup: team[pick[4]].clone(),
    };
    (map, key, score)
  }

  fn best_perm_pref_first<R: Rng + ?Sized>(
    &self,
    team: &[Player; 5],
    mut rng: Option<&mut R>,
    priority_main: &HashSet<u32>,
  ) -> ([usize; 5], PrefKey, f64) {
    let roles_arr = Role::ALL;
    let idxs = [0usize, 1, 2, 3, 4];

    let mut best_off = usize::MAX;
    let mut best_score = f64::NEG_INFINITY;
    let mut candidates: Vec<([usize; 5], PrefKey, f64)> = Vec::new();

    for perm in permutations(idxs) {
      let mut off = 0usize;
      let mut effw = [0.0f64; 5];

      for (ri, &role) in roles_arr.iter().enumerate() {
        let p = &team[perm[ri]];
        let r = self.score(p, role, priority_main);
        effw[ri] = r.effw;
        off += r.off_delta;
      }

      let score = match self.cfg.eval.score {
        TeamScore::Softmax { tau } => softmax_score(&effw, tau),
        TeamScore::TopK { k } => topk_score(&effw, k),
      };

      if off < best_off {
        best_off = off;
        best_score = score;
        candidates.clear();
        candidates.push((perm, PrefKey { off_count: off }, score));
      } else if off == best_off {
        if score > best_score {
          best_score = score;
        }
        candidates.push((perm, PrefKey { off_count: off }, score));
      }
    }

    let mut filtered: Vec<([usize; 5], PrefKey, f64)> = candidates
      .into_iter()
      .filter(|(_, _, s)| *s >= best_score - self.cfg.eval.score_margin)
      .collect();

    if filtered.len() <= 1 || rng.is_none() {
      let (perm, key, score) = filtered
        .into_iter()
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap();
      return (perm, key, score);
    }

    let r = rng.as_deref_mut().unwrap();
    let idx = r.random_range(0..filtered.len());
    let (perm, key, score) = filtered.swap_remove(idx);

    (perm, key, score)
  }


  // /// 分割評価用（割当は返さずスコアだけ）
  // pub fn best_score(&self, team: &[Player;5]) -> (PrefKey, f64) {
  //   let (_pick, key, score) = self.best_perm_pref_first(team);
  //   (key, score)
  // }

  pub fn role_effw(&self, assigned: &RoleMap<Player>, priority_main: &HashSet<u32>) -> [f64; 5] {
    let pairs: [(Role, &Player); 5] = [
      (Role::Top, &assigned.top),
      (Role::Jg,  &assigned.jg),
      (Role::Mid, &assigned.mid),
      (Role::Adc, &assigned.adc),
      (Role::Sup, &assigned.sup),
    ];

    let mut effw = [0.0; 5];
    for (i, (role, p)) in pairs.iter().enumerate() {
      let r = self.score(p, *role, priority_main);
      effw[i] = r.effw;
    }
    effw
  }

  fn score(
    &self,
    p: &Player,
    role: Role,
    priority_main: &HashSet<u32>,
  ) -> RoleEval {
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
    let mut pen = self.cfg.penalty
      .total_penalty(p, role) as f64
      * penalty_multiplier_from_z(z, self.cfg.eval.flex_bias_alpha);

    // ★ 優遇対象 + メインロールならペナルティを少し下げる
    if is_main && priority_main.contains(&p.id) {
      pen -= self.cfg.eval.priority_main_bonus;
      if pen < 0.0 {
        pen = 0.0;
      }
    }

    let eff = base - pen;
    let effw = eff * self.cfg.role_weight.weight(&role);

    RoleEval {
      effw,
      off_delta,
      sub_delta,
      off_mmr_neg_delta,
    }
  }
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PrefKey {
  pub off_count: usize,
}

fn softmax_score(e:&[f64;5], tau:f64)->f64 {
  let t = tau.max(1.0);
  let m = e.iter().cloned().fold(f64::NEG_INFINITY,f64::max);
  let exps:Vec<f64> = e.iter().map(|x|((x - m) / t).exp()).collect();
  let den: f64 = exps.iter().sum();
  e.iter().zip(&exps).map(|(x,w)| x * w).sum::<f64>() / den
}

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