use crate::pipeline::states::{AssignedTeams, Lobby};

pub mod default;

pub trait TeamSolver {
  fn solve(&self, lobby: &Lobby) -> AssignedTeams;
}