use crate::domain::states::{AssignedTeams, Lobby};

pub trait TeamOptimizer {
  fn optimize(&self, lobby: &Lobby) -> AssignedTeams;
}


pub struct DefaultSolver<O: TeamOptimizer> {
  optimizer: O
}

impl<O: TeamOptimizer> DefaultSolver<O> {
  pub fn new(optimizer: O) -> Self {
    Self { optimizer }
  }
}

impl<O: TeamOptimizer> DefaultSolver<O> {
  pub fn solve(&self, lobby: &Lobby) -> AssignedTeams {
    self.optimizer.optimize(lobby)
  }
}