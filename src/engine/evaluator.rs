use crate::domain::states::Lobby;
use crate::domain::types::{Player, Role, RoleMap};
use crate::param::penalty_matrix::PenaltyMatrix;
use crate::param::role_weights::RoleWeights;
use crate::param::eval::{EvalContext, LobbyStats, TeamScore, penalty_multiplier_from_z};


#[derive(Clone)]
pub struct Evaluator { pub cfg: EvalContext }


pub struct EvalState { pub lobby: LobbyStats }


impl Evaluator {
  pub fn new(cfg: EvalContext) -> Self {
    Self { cfg }
  }
  pub fn state_for(&self, lobby: &Lobby) -> EvalState {
    EvalState { lobby: LobbyStats::from_lobby(lobby, &self.cfg.eval.mmr) }
  }


  /// 希望>サブ>オフ（オフは高MMR担当）で最良割当とスコア
  pub fn best_assignment(&self, st: &EvalState, team: &[Player;5]) -> (RoleMap<Player>, PrefKey, f64) {
    let (pick, key, score) = best_perm_pref_first(team, self, st);
    let map = RoleMap { top: team[pick[0]].clone(), jg: team[pick[1]].clone(), mid: team[pick[2]].clone(), adc: team[pick[3]].clone(), sup: team[pick[4]].clone() };
    (map, key, score)
  }


  /// 分割評価用（割当は返さずスコアだけ）
  pub fn best_score(&self, st: &EvalState, team: &[Player;5]) -> (PrefKey, f64) {
    let (_pick, key, score) = best_perm_pref_first(team, self, st);
    (key, score)
  }
    let roles_arr = Role::ALL;


  for perm in permutations(idxs) {
    let mut off = 0usize;
    let mut sub = 0usize;
    let mut off_mmr_neg_sum = 0f64;
    let mut effw = [0f64;5];


    for (ri, &role) in roles_arr.iter().enumerate() {
      let p = &team[perm[ri]];
      let base = ev.cfg.eval.mmr.calculate(&p.rank);
      let is_main = role == p.main_role;
      let is_sub = !is_main && p.sub_role.contains(&role);
      if !is_main && !is_sub {
        off += 1; off_mmr_neg_sum += -base;
      }
      if is_sub {
        sub += 1;
      }

      let z = ev.cfg.lobby.z_from(base);
      let pen = ev.cfg.penalty.total_penalty(p, role)
        as f64 * penalty_multiplier_from_z(z, ev.cfg.eval.flex_bias_alpha);

      let eff = base - pen;
      effw[ri] = eff * ev.cfg.role_weight.weight(&role);
    }

    let score = match ev.cfg.eval.score { TeamScore::Softmax{ tau } => softmax_score(&effw, tau), TeamScore::TopK{ k } => topk_score(&effw, k) };

    let key = PrefKey { off_count: off, off_mmr_neg_sum, sub_count: sub };

    let better =
    key.off_count < best_key.off_count ||
    (key.off_count == best_key.off_count && key.off_mmr_neg_sum.total_cmp(&best_key.off_mmr_neg_sum).is_lt()) ||
    (key.off_count == best_key.off_count && key.off_mmr_neg_sum.total_cmp(&best_key.off_mmr_neg_sum).is_eq() && key.sub_count < best_key.sub_count) ||
    (key.off_count == best_key.off_count && key.off_mmr_neg_sum.total_cmp(&best_key.off_mmr_neg_sum).is_eq() && key.sub_count == best_key.sub_count && score > best_score);


    if better {
      best_pick = perm;
      best_key = key;
      best_score = score;
    }
  }
  (best_pick, best_key, best_score)
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