use rand::{rngs::StdRng, Rng, SeedableRng};
use crate::pipeline::TeamOptimizer;
use crate::domain::states::{AssignedTeams, Lobby, Team};
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

    let mut best_power_diff = f64::INFINITY;
    let mut best_lane_diff  = f64::INFINITY;

    // 候補は「(チーム構成, power_diff, lane_diff)」で持つ
    let mut cands: Vec<(AssignedTeams, f64, f64)> = Vec::new();

    for mask in 0u16..(1 << 10) {
      if mask.count_ones() != 5 { continue; }

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

      // 各チーム内で「希望>サブ>オフ」＋Softmax/TopK でベスト割当
      let (red_map,  key_r, _sr) = self.evaluator.best_assignment(&red);
      let (blue_map, key_b, _sb) = self.evaluator.best_assignment(&blue);

      // 各ロールの effw を取得
      let eff_r = self.evaluator.role_effw(&red_map);
      let eff_b = self.evaluator.role_effw(&blue_map);

      // 総合力（単純な和）とレーンごとの差
      let power_r: f64 = eff_r.iter().sum();
      let power_b: f64 = eff_b.iter().sum();
      let power_diff = (power_r - power_b).abs();

      let lane_diff: f64 = eff_r.iter()
        .zip(eff_b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

      let red_team = Team {
        players: red_map,
        main: (5 - key_r.sub_count - key_r.off_count),
        sub: key_r.sub_count,
        off: key_r.off_count,
        power: power_r,
      };
      let blue_team = Team {
        players: blue_map,
        main: (5 - key_b.sub_count - key_b.off_count),
        sub: key_b.sub_count,
        off: key_b.off_count,
        power: power_b,
      };
      let assigned = AssignedTeams { red: red_team, blue: blue_team };

      // まずは power_diff を最優先で最小化
      if power_diff + 1e-9 < best_power_diff {
        best_power_diff = power_diff;
        best_lane_diff  = lane_diff;
        cands.clear();
        cands.push((assigned, power_diff, lane_diff));
      }
      // 「power_diff がベスト＋マージン以内」ならレーン差も見て候補に入れる
      else if (power_diff - best_power_diff).abs() <= self.evaluator.cfg.eval.power_margin {
        if lane_diff + 1e-9 < best_lane_diff {
          best_lane_diff = lane_diff;
          cands.clear();
          cands.push((assigned, power_diff, lane_diff));
        } else if (lane_diff - best_lane_diff).abs() <= self.evaluator.cfg.eval.lane_margin {
          // ほぼ同じくらい良いので候補に追加（ランダム性UP）
          cands.push((assigned, power_diff, lane_diff));
        }
      }
    }

    if cands.is_empty() {
      panic!("no candidate split");
    }

    // 候補から乱択する
    if let Some(mut rng) = self.rng.clone() {
      let idx = rng.random_range(0..cands.len());
      return cands[idx].0.clone();
    }

    // RNGが無い場合はとりあえず最初の候補
    cands[0].0.clone()
  }
}