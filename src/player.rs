#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
  Top,
  Jg,
  Mid,
  Adc,
  Sup,
}

impl Role {
  const All: [Role; 5] = [Role::Top, Role::Jg, Role::Mid, Role::Adc, Role::Sup];

  fn name(&self) -> &'static str {
    match self {
      Role::Top => "Top",
      Role::Jg => "Jg",
      Role::Mid => "Mid",
      Role::Adc => "Adc",
      Role::Sup => "Sup",
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Tier {
  Iron,
  Bronze,
  Silver,
  Gold,
  Platinum,
  Diamond,
  Master,
  Grandmaster,
  Challenger,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Division {
  I,
  II,
  III,
  IV,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Rank {
  tier: Tier,
  division: Option<Division>,
  lp: usize, // master+ lp
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
  pub name: String,
  pub rank: Rank,
  pub main_role: Role,
  pub sub_role: Vec<Role>,
}