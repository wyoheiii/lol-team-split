use crate::domain::{types::{RoleMap, Side}, Player};



#[derive(Clone, Debug)]
pub struct Lobby { players: [Player; 10] }

impl Lobby {
  pub fn new(players: [Player; 10]) -> Self {
    assert!(players.len() == 10);
    Self { players }
  }

  pub fn players(&self) -> &[Player; 10] {
    &self.players
  }
}

#[derive(Clone, Debug)]
struct SplitTeams { a: [Player; 5], b: [Player; 5] }

impl SplitTeams {
  pub fn new(a: [Player; 5], b: [Player; 5]) -> Self {
    assert!(a.len() == 5);
    assert!(b.len() == 5);
    Self { a, b }
  }

  pub fn team_a(&self) -> &[Player; 5] {
    &self.a
  }

  pub fn team_b(&self) -> &[Player; 5] {
    &self.b
  }
}

#[derive(Clone, Debug)]
struct TeamAssignedNoSide { pub members: RoleMap<Player> }

#[derive(Clone, Debug)]
pub struct UnlabeledAssignedTeams {
  pub a: TeamAssignedNoSide,
  pub b: TeamAssignedNoSide,
}

#[derive(Clone, Debug)]
pub struct TeamAssigned {
  pub side: Side,
  pub members: RoleMap<Player>,
}

#[derive(Clone, Debug)]
pub struct AssignedTeams {
  pub blue: TeamAssigned,
  pub red: TeamAssigned,
}