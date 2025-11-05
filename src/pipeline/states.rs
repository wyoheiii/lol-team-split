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
pub struct SplitTeams { red: [Player; 5], blue: [Player; 5] }

impl SplitTeams {
  pub fn new(red: [Player; 5], blue: [Player; 5]) -> Self {
    assert!(red.len() == 5);
    assert!(blue.len() == 5);
    Self { red, blue }
  }

  pub fn red(&self) -> &[Player; 5] {
    &self.red
  }

  pub fn blue(&self) -> &[Player; 5] {
    &self.blue
  }
}

#[derive(Clone, Debug)]
pub struct AssignedTeams {
  pub red: RoleMap<Player>,
  pub blue: RoleMap<Player>,
}