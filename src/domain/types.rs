use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Role {
  Top,
  Jg,
  Mid,
  Adc,
  Sup,
}

impl Role {
  pub const All: [Role; 5] = [Role::Top, Role::Jg, Role::Mid, Role::Adc, Role::Sup];

}

impl fmt::Display for Role {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Role::Top => write!(f, "Top"),
      Role::Jg => write!(f, "Jng"),
      Role::Mid => write!(f, "Mid"),
      Role::Adc => write!(f, "ADC"),
      Role::Sup => write!(f, "Sup"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum TierBelowMaster { Iron, Bronze, Silver, Gold, Platinum, Emerald, Diamond }

impl fmt::Display for TierBelowMaster {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TierBelowMaster::Iron => write!(f, "Iron"),
      TierBelowMaster::Bronze => write!(f, "Bronze"),
      TierBelowMaster::Silver => write!(f, "Silver"),
      TierBelowMaster::Gold => write!(f, "Gold"),
      TierBelowMaster::Platinum => write!(f, "Platinum"),
      TierBelowMaster::Emerald => write!(f, "Emerald"),
      TierBelowMaster::Diamond => write!(f, "Diamond"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum MasterLeague { Master, Grandmaster, Challenger }

impl fmt::Display for MasterLeague {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MasterLeague::Master => write!(f, "Master"),
      MasterLeague::Grandmaster => write!(f, "Grandmaster"),
      MasterLeague::Challenger => write!(f, "Challenger"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Division {
  I,
  II,
  III,
  IV,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Rank {
  BelowMaster {
    tier: TierBelowMaster,
    division: Division,
  },
  MasterLeague { tier: MasterLeague , lp: u32 },
}

impl fmt::Display for Rank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Rank::BelowMaster { tier, division } => {
        write!(f, "{} {:?}", tier, division)
      }
      Rank::MasterLeague { tier, lp } => {
        write!(f, "{} {}LP", tier, lp)
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
  pub name: String,
  pub rank: Rank,
  pub main_role: Role,
  pub sub_role: Vec<Role>,
}

impl fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {},",
      self.name,
      self.rank,
    )
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Side {
  Blue,
  Red,
}

impl fmt::Display for Side {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Side::Blue => write!(f, "Blue"),
      Side::Red => write!(f, "Red"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct RoleMap<T> {
  pub top: T,
  pub jg: T,
  pub mid: T,
  pub adc: T,
  pub sup: T,
}

impl<T> RoleMap<T> {
  pub fn iter(&self) -> [(&T, Role); 5] {
      [
        (&self.top, Role::Top),
        (&self.jg, Role::Jg),
        (&self.mid, Role::Mid),
        (&self.adc, Role::Adc),
        (&self.sup, Role::Sup),
      ]
    }
}