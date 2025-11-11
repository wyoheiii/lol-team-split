use crate::{domain::states::Lobby, param::{mmr::{self, MMR}, penalty_matrix::PenaltyMatrix, role_weights::RoleWeights}};



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
  // 平均
  pub mean: f64,
  // 標準偏差
  pub std: f64,
}

impl LobbyStats {
pub fn from_lobby(lobby: &Lobby, mmr: &MMR) -> Self {
    let mut vals: Vec<f64> = lobby.players().iter().map(|p| mmr.calculate(&p.rank)).collect();
    if vals.is_empty() {
      return Self{ mean:0.0, std:1.0 };
    }

    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let std = var.sqrt();
    Self { mean, std: if std==0.0 { 1.0 } else { std } }
  }

  // プレイヤーの相対的な位置(z score)を算出
  pub fn z_from(&self,  base: f64) -> f64 {
    (base - self.mean) / self.std
  }
}

// zスコアからペナルティ倍率を算出
pub fn penalty_multiplier_from_z(z: f64, alpha: f64) -> f64 {
  (1.0 + (-z) * alpha).clamp(0.6, 1.8)
}


#[derive(Debug, Clone)]
pub struct EvalContext {
  pub penalty: PenaltyMatrix,
  pub role_weight: RoleWeights,
  pub eval: Eval,
  pub lobby: LobbyStats,
}

impl EvalContext {
  pub fn new(lobby: &Lobby)->Self {
    Self {
      penalty: PenaltyMatrix::default(),
      role_weight: RoleWeights::default(),
      eval: Eval::default(),
      lobby: LobbyStats::from_lobby(lobby, &MMR::default()),
    }
  }
}