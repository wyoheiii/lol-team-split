use crate::pipeline::states::{SplitTeams, AssignedTeams};

pub mod random;

pub trait RoleAssigner {
  fn assign(&self, split: SplitTeams) -> AssignedTeams;
}