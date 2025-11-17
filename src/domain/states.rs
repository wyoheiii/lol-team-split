use crate::domain::{types::{RoleMap}, Player};



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
pub struct AssignedTeams {
  pub red: RoleMap<Player>,
  pub blue: RoleMap<Player>,
}