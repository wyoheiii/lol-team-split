use std::collections::HashMap;
use crate::domain::{Division, Rank, TierBelowMaster};

#[derive(Debug, Clone, PartialEq)]
pub struct MMR {
  master_base_mmr: i32,
  ip_scale: f64,

  tier_table: HashMap<(TierBelowMaster, Division), i32>,
}


// バランスが悪い場合調整する。

impl Default for MMR {
  fn default() -> Self {
    let mut table = HashMap::new();

  let mut p = |tier: TierBelowMaster, division: Division, mmr: i32| {
    table.insert((tier, division), mmr);
  };

  let tiers = [
    (TierBelowMaster::Iron, 200),
    (TierBelowMaster::Bronze, 300),
    (TierBelowMaster::Silver, 400),
    (TierBelowMaster::Gold, 500),
    (TierBelowMaster::Platinum, 600),
    (TierBelowMaster::Emerald, 700),
    (TierBelowMaster::Diamond, 800),
  ];

  let div_offsets = [
    (Division::IV, 0),
    (Division::III, 25),
    (Division::II, 50),
    (Division::I, 75),
  ];

  for (tier, base) in tiers {
    for (div, off) in div_offsets {
      p(tier, div, base + off);
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
  pub fn calculate(&self, rank: &Rank) -> i32 {
    match rank {
      Rank::BelowMaster { tier, division } => {
        *self.tier_table.get(&(*tier, *division)).unwrap()
      }
      Rank::MasterLeague { tier, lp } => {
        let base = self.master_base_mmr;
        let extra = (*lp as f64 * self.ip_scale) as i32;
        base + extra
      }
    }
  }
}