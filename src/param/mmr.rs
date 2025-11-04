use std::collections::HashMap;
use crate::domain::{Tier, Division, Rank};

#[derive(Debug, Clone, PartialEq)]
struct MMR {
  master_base_mmr: i32,
  ip_scale: f64,

  tier_table: HashMap<(Tier, Option<Division>), i32>,
}


// バランスが悪い場合調整する。

impl Default for MMR {
  fn default() -> Self {
    let mut table = HashMap::new();

  let mut p = |tier: Tier, division: Option<Division>, mmr: i32| {
    table.insert((tier, division), mmr);
  };

  let tiers = [
    (Tier::Iron, 200),
    (Tier::Bronze, 300),
    (Tier::Silver, 400),
    (Tier::Gold, 500),
    (Tier::Platinum, 600),
    (Tier::Emerald, 700),
    (Tier::Diamond, 800),
  ];

  let div_offsets = [
    (Division::IV, 0),
    (Division::III, 25),
    (Division::II, 50),
    (Division::I, 75),
  ];

  for (tier, base) in tiers {
    for (div, off) in div_offsets {
      p(tier, Some(div), base + off);
    }
  }

  MMR {
    master_base_mmr: 900,
    ip_scale: 1.0, // master↑のMMR増加量に対する倍率 要調整
    tier_table: table,
  }
  }
}

impl MMR {
  pub fn calculate_mmr(&self, rank: &Rank) -> i32 {
    match rank.tier {
      Tier::Master | Tier::Grandmaster | Tier::Challenger =>
      self.master_base_mmr + (rank.lp as f64 * self.ip_scale) as i32,
      _ => {
        *self.tier_table.get(&(rank.tier, rank.division)).unwrap()
      }
    }
  }
}