use crate::pipeline::states::{Lobby, SplitTeams};

pub mod random;

trait TeamSplitter {
  fn split(&self, lobby: &Lobby) -> SplitTeams;
}