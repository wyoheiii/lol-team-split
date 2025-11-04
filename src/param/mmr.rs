use std::collections::HashMap;
use crate::domain::{Tier, Division, Rank};

#[derive(Debug, Clone, PartialEq)]
struct MMRConfig {
  master_base_mmr: i32,
  ip_scale: f64,

  tier_table: HashMap<(Tier, Option<Division>), i32>,
}


// バランスが悪い場合調整する。
fn build_default_mmr_config() -> MMRConfig {
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
    (Division::III, 20),
    (Division::II, 40),
    (Division::I, 60),
  ];

  for (tier, base) in tiers {
    for (div, off) in div_offsets {
      p(tier, Some(div), base + off);
    }
  }

  MMRConfig {
    master_base_mmr: 900,
    ip_scale: 1.0, // master↑のMMR増加量に対する倍率 要調整
    tier_table: table,
  }
}

fn base_mmr(rank: &Rank, config: &MMRConfig) -> i32 {
  match rank.tier {
    Tier::Master | Tier::Grandmaster | Tier::Challenger =>
    config.master_base_mmr + (rank.lp as f64 * config.ip_scale) as i32,
    _ => {
      *config.tier_table.get(&(rank.tier, rank.division)).unwrap()
    }
  }
}