use crate::domain::states::{SplitTeams, AssignedTeams};

pub mod random;

pub trait RoleAssigner {
  fn assign(&self, split: SplitTeams) -> AssignedTeams;
}