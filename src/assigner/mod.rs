use crate::pipeline::states::{SplitTeams, AssignedTeams};

pub mod random;

trait RoleAssigner {
  fn assign(&self, split: SplitTeams)-> AssignedTeams;
}