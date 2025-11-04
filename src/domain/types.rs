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

  pub fn name(&self) -> &'static str {
    match self {
      Role::Top => "Top",
      Role::Jg => "Jg",
      Role::Mid => "Mid",
      Role::Adc => "Adc",
      Role::Sup => "Sup",
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
  pub name: String,
  pub rank: Rank,
  pub main_role: Role,
  pub sub_role: Vec<Role>,
}