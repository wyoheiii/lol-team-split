use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
      Role::Jg => write!(f, "Jg"),
      Role::Mid => write!(f, "Mid"),
      Role::Adc => write!(f, "ADC"),
      Role::Sup => write!(f, "Sup"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Tier {
  Iron,
  Bronze,
  Silver,
  Gold,
  Platinum,
  Emerald,
  Diamond,
  Master,
  Grandmaster,
  Challenger,
}

impl fmt::Display for Tier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Tier::Iron => write!(f, "Iron"),
      Tier::Bronze => write!(f, "Bronze"),
      Tier::Silver => write!(f, "Silver"),
      Tier::Gold => write!(f, "Gold"),
      Tier::Platinum => write!(f, "Platinum"),
      Tier::Emerald => write!(f, "Emerald"),
      Tier::Diamond => write!(f, "Diamond"),
      Tier::Master => write!(f, "Master"),
      Tier::Grandmaster => write!(f, "Grandmaster"),
      Tier::Challenger => write!(f, "Challenger"),
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
pub struct Rank {
  pub tier: Tier,
  pub division: Option<Division>,
  pub lp: usize, // master+ lp
}

impl fmt::Display for Rank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.tier)
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