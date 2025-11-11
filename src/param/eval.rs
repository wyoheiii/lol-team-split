use crate::{param::mmr, domain::states::Lobby};



#[derive(Debug, Clone)]
pub struct Eval {
  pub softmax_tau: f64,
  pub mmr : mmr::MMR,
  pub flex_bias_alpha: f64,
}

impl Default for Eval {
  fn default() -> Self {
    Eval {
      softmax_tau: 150.0,
      mmr: mmr::MMR::default(),
      flex_bias_alpha: 0.15,
    }
  }
}

#[derive(Debug, Clone)]
pub struct LobbyStats {
  pub mean: f64,
  pub std: f64,
}

impl LobbyStats {
pub fn from_players(players: &Lobby) -> Self {
let mut vals: Vec<f64> = players.players().iter().map(|p| abs_mmr_from_rank(p.rank, &RatingModelCfg::default())).collect();
if vals.is_empty() { return Self{ mean:0.0, std:1.0 }; }
let n = vals.len() as f64;
let mean = vals.iter().sum::<f64>() / n;
let var = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
let std = var.sqrt();
Self { mean, std: if std==0.0 { 1.0 } else { std } }
}
}