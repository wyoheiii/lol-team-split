use std::cell::RefCell;

use rand::{rngs::StdRng, Rng, SeedableRng, rng};
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
    fn build_team(&self, team_players: &[Player; 5]) -> Team {
      let mut rng = self.rng.borrow_mut();
      let (role_map, key, score) = self.evaluator.best_assignment(team_players, Some(&mut rng));
      let effs = self.evaluator.role_effw(&role_map);
      let sum = effs.iter().sum::<f64>();

      Team {
        players: role_map,
        off: key.off_count,
        //power: score, // 表示用のTeam Power（SoftmaxなりTopKなり）
        power: sum,
      }
    }
}


impl TeamOptimizer for JointEnumeratingOptimizer {
  fn optimize(&self, lobby: &Lobby) -> AssignedTeams {
    let players = lobby.players();

    // ① まずMMRの和だけでチーム分割を決める
    let (red_idx, blue_idx) = self.best_mmr_split(players);

    // ② index -> 実際の Player 配列へ
    let red_vec: Vec<Player> = red_idx.iter().map(|&i| players[i].clone()).collect();
    let blue_vec: Vec<Player> = blue_idx.iter().map(|&i| players[i].clone()).collect();
    let red_arr: [Player; 5] = red_vec.try_into().unwrap();
    let blue_arr: [Player; 5] = blue_vec.try_into().unwrap();

    // ③ 各チーム内で「希望＞サブ＞オフ＋低レート優先＋ランダム性」でロール割り当て
    let red_team  = self.build_team(&red_arr);
    let blue_team = self.build_team(&blue_arr);

    AssignedTeams { red: red_team, blue: blue_team }
  }
}