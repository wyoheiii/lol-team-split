use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use rand::{rngs::StdRng, Rng, SeedableRng, rng};
use crate::domain::Role;
use crate::pipeline::TeamOptimizer;
use crate::domain::states::{AssignedTeams, Lobby, Team};
use crate::domain::types::Player;
use crate::engine::evaluator::{Evaluator};


pub struct JointEnumeratingOptimizer {
  pub evaluator: Evaluator,
  pub rng: RefCell<StdRng>,

}
impl JointEnumeratingOptimizer {
  pub fn new(evaluator: Evaluator, seed: Option<u64>) -> Self {
    let rng = match seed {
      Some(s) => StdRng::seed_from_u64(s),
      None => StdRng::from_rng(&mut rng()),
    };
    Self { evaluator, rng: RefCell::new(rng) }
  }

  /// MMRの和だけで「一番MMR差が小さい 5-5 分割」を探す
    fn best_mmr_split(&self, players: &[Player]) -> (Vec<usize>, Vec<usize>) {
      let mmr = &self.evaluator.cfg.eval.mmr;
      assert!(players.len() == 10, "best_mmr_split は 10 人前提");

      #[derive(Clone)]
      struct Cand {
        red: Vec<usize>,
        blue: Vec<usize>,
        sum_diff: f64,
        shape_diff: f64,
      }

      let mut cands: Vec<Cand> = Vec::new();

      // 5人ずつになるマスクを全探索
      for mask in 0u16..(1 << 10) {
        if mask.count_ones() != 5 {
          continue;
        }

        let mut red_idx = Vec::with_capacity(5);
        let mut blue_idx = Vec::with_capacity(5);
        let mut red_vals = Vec::with_capacity(5);
        let mut blue_vals = Vec::with_capacity(5);

        for i in 0..10 {
          let v = mmr.calculate(&players[i].rank);
          if (mask >> i) & 1 == 1 {
            red_idx.push(i);
            red_vals.push(v);
          } else {
            blue_idx.push(i);
            blue_vals.push(v);
          }
        }

        // 念のため
        if red_idx.len() != 5 || blue_idx.len() != 5 {
          continue;
        }

        // 降順にソート（強い順）
        red_vals.sort_by(|a, b| b.partial_cmp(a).unwrap());
        blue_vals.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let sum_red: f64 = red_vals.iter().sum();
        let sum_blue: f64 = blue_vals.iter().sum();
        let sum_diff = (sum_red - sum_blue).abs();

        // MMRベクトル形状の差分
        let weights = [2.0, 1.5, 1.0, 0.7, 0.5];
        let mut shape_diff = 0.0;
        for j in 0..5 {
          let d = (red_vals[j] - blue_vals[j]).abs();
          shape_diff += weights[j] * d;
        }

        cands.push(Cand {
          red: red_idx,
          blue: blue_idx,
          sum_diff,
          shape_diff,
        });
      }

      if cands.is_empty() {
        panic!("no candidate split");
      }

      // まず合計MMR差が最小の値を取る
      let best_sum = cands
        .iter()
        .map(|c| c.sum_diff)
        .fold(f64::INFINITY, f64::min);

      // best_sum + margin 以内のものを候補に
      let sum_margin = 30.0; // ← 好みで [10〜40] くらい調整
      let near_sum: Vec<Cand> = cands
        .into_iter()
        .filter(|c| c.sum_diff <= best_sum + sum_margin)
        .collect();

      // その中で shape_diff の最小値を取る
      let best_shape = near_sum
        .iter()
        .map(|c| c.shape_diff)
        .fold(f64::INFINITY, f64::min);

      // best_shape + margin 以内のものを最終候補に
      let shape_margin = 50.0; // ← ここで「上位帯の割り方のバリエーション」を調整
      let final_cands: Vec<Cand> = near_sum.clone()
        .into_iter()
        .filter(|c| c.shape_diff <= best_shape + shape_margin)
        .collect();

      let list = if final_cands.is_empty() {
        // 念のためフォールバック（ほぼ起きない想定）
        near_sum
      } else {
        final_cands
      };

      let mut rng = self.rng.borrow_mut();
      let idx = rng.random_range(0..list.len());
      let chosen = &list[idx];

      (chosen.red.clone(), chosen.blue.clone())
    }

    /// 5人に対してロール割り当て＋Team構造体を作る
    fn build_team(&self, team_players: &[Player; 5], priority_main: &HashSet<u32>) -> Team {
      let mut rng = self.rng.borrow_mut();
      let (role_map, key, score) = self.evaluator.best_assignment(team_players, Some(&mut rng), priority_main);
      let effs = self.evaluator.role_effw(&role_map, priority_main);
      let sum = effs.iter().sum::<f64>();

      Team {
        players: role_map,
        off: key.off_count,
        //power: score, // 表示用のTeam Power（SoftmaxなりTopKなり）
        power: sum,
      }
    }

  fn pick_priority_main(
    &self,
    lobby: &Lobby,
  ) -> HashSet<u32> {
    let players = lobby.players();
    let mmr = &self.evaluator.cfg.eval.mmr;

    // main_role ごとにグループ化
    let mut by_main: HashMap<Role, Vec<&Player>> = HashMap::new();
    for p in players {
      by_main.entry(p.main_role).or_default().push(p);
    }

    // ★ レーン被り組だけ候補にする
    let mut candidates: Vec<&Player> = Vec::new();
    for (_role, group) in &by_main {
      if group.len() >= 2 {
        candidates.extend(group.iter().copied());
      }
    }
    if candidates.is_empty() {
      return HashSet::new();
    }

    // 低レートほど重くする（z がマイナスほど重く）
    let mut weights: Vec<f64> = Vec::with_capacity(candidates.len());
    for p in &candidates {
      let base = mmr.calculate(&p.rank);
      let z = self.evaluator.cfg.lobby.z_from(base); // 平均より低いと z<0
      // ざっくり: z=-2 → w≈1.5, z=0→1.0, z=+2→0.5
      let w = ((1.0 - 0.25 * z) as f64).clamp(0.2, 2.0);
      weights.push(w);
    }

    let k = candidates.len().min(3); // 最大3人
    let mut chosen = HashSet::new();
    let mut idxs: Vec<usize> = (0..candidates.len()).collect();
    let mut rng = self.rng.borrow_mut();

    for _ in 0..k {
      if idxs.is_empty() {
        break;
      }

      // 重み付きサンプリング (without replacement)
      let mut total = 0.0;
      for &i in &idxs {
        total += weights[i].max(0.0);
      }
      if total <= 0.0 {
        break;
      }

      let mut r = rng.gen_range(0.0..total);
      let mut pick_pos = 0;
      for (pos, &i) in idxs.iter().enumerate() {
        let w = weights[i].max(0.0);
        if r <= w {
          pick_pos = pos;
          break;
        }
        r -= w;
      }
      let i = idxs.remove(pick_pos);
      let pid  = candidates[i].id.clone(); // ← 実際のフィールドに合わせて
      chosen.insert(pid);
    }

    chosen
  }
}


impl TeamOptimizer for JointEnumeratingOptimizer {
  fn optimize(&self, lobby: &Lobby) -> AssignedTeams {
    let players = lobby.players();


    // ロビー全体から「レーン被り＋低レート」を重み付きで最大3人選ぶ
    let priority_main = self.pick_priority_main(lobby);

    // ① まずMMRの和だけでチーム分割を決める
    let (red_idx, blue_idx) = self.best_mmr_split(players);

    // ② index -> 実際の Player 配列へ
    let red_vec: Vec<Player> = red_idx.iter().map(|&i| players[i].clone()).collect();
    let blue_vec: Vec<Player> = blue_idx.iter().map(|&i| players[i].clone()).collect();
    let red_arr: [Player; 5] = red_vec.try_into().unwrap();
    let blue_arr: [Player; 5] = blue_vec.try_into().unwrap();

    // ③ 各チーム内で「希望＞サブ＞オフ＋低レート優先＋ランダム性」でロール割り当て
    let red_team  = self.build_team(&red_arr, &priority_main);
    let blue_team = self.build_team(&blue_arr, &priority_main);

    AssignedTeams { red: red_team, blue: blue_team }
  }
}