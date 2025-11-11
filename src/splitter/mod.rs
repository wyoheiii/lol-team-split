use crate::domain::states::{Lobby, SplitTeams};

pub mod random;

pub trait TeamSplitter {
  fn split(&self, lobby: &Lobby) -> SplitTeams;
}