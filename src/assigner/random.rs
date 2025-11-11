use crate::{assigner::RoleAssigner, domain::{types::RoleMap, Player}, domain::states::{AssignedTeams, SplitTeams}};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
pub struct RandomRoleAssigner { seed: u64 }

impl RandomRoleAssigner {
  pub fn new(seed: u64) -> Self {
    Self { seed }
  }
}

impl RoleAssigner for RandomRoleAssigner {
  fn assign(&self, split: SplitTeams)-> AssignedTeams {
    let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);

    let assign_role = |p: &[Player; 5], rng: &mut StdRng| -> RoleMap<Player> {
      let mut shuffled = p.clone();
      shuffled.shuffle(rng);

      let m = RoleMap {
        top: shuffled[0].clone(),
        jg: shuffled[1].clone(),
        mid: shuffled[2].clone(),
        adc: shuffled[3].clone(),
        sup: shuffled[4].clone(),
      };

      m
    };

    AssignedTeams {
      red: assign_role(split.red(), &mut rng),
      blue: assign_role(split.blue(), &mut rng),
    }

  }
}