use crate::domain::states::{AssignedTeams, Lobby};
use crate::solver::TeamSolver;
use crate::splitter::TeamSplitter;
use crate::assigner::RoleAssigner;

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