use crate::domain::states::{AssignedTeams, Lobby, SplitTeams};




pub trait RoleAssigner {
  fn assign(&self, split: SplitTeams) -> AssignedTeams;
}

pub trait TeamSplitter {
  fn split(&self, lobby: &Lobby) -> SplitTeams;
}

pub trait TeamSolver {
  fn solve(&self, lobby: &Lobby) -> AssignedTeams;
}


pub struct DefaultSolver<S: TeamSplitter, A: RoleAssigner> {
  splitter: S,
  assigner: A,
}



impl<S: TeamSplitter, A: RoleAssigner> DefaultSolver<S, A> {
    pub fn new(splitter: S, assigner: A) -> Self {
      Self { splitter, assigner }
    }
}

impl<S: TeamSplitter, A: RoleAssigner> TeamSolver for DefaultSolver<S, A> {
  fn solve(&self, lobby: &Lobby) -> AssignedTeams {
    let split = self.splitter.split(lobby);
    self.assigner.assign(split)
  }
}