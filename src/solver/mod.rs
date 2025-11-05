use crate::pipeline::states::{AssignedTeams, Lobby};

mod default;

trait TeamSolver {
  fn solve(&self, lobby: &Lobby) -> AssignedTeams;
}