use crate::domain::states::{AssignedTeams, Lobby, SplitTeams};




pub trait RoleAssigner {
  fn assign(&self, split: SplitTeams) -> AssignedTeams;
}

pub trait TeamSplitter {
  fn split(&self, lobby: &Lobby) -> SplitTeams;
}

pub trait TeamOptimizer {
  fn optimize(&self, lobby: &Lobby) -> AssignedTeams;
}

pub trait TeamSolver {
  fn solve(&self, lobby: &Lobby) -> AssignedTeams;
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