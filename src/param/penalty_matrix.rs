use std::{collections::HashMap};

use crate::domain::{Player, Role};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PenaltyRolePair {
  pub main: Role,
  pub assign: Role,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PenaltyMatrix {
  sub_role_penalty: i32,
  off_role_penalty: i32,

  // 特定のロール間の遷移ペナルティ
  role_transition_penalty_map: HashMap<PenaltyRolePair, i32>,
}


impl Default for PenaltyMatrix {
  fn default() -> Self {
    let mut transition_penalty = HashMap::new();

    transition_penalty.insert(PenaltyRolePair { main:Role::Sup, assign: Role::Top }, 150);
    transition_penalty.insert(PenaltyRolePair { main:Role::Sup, assign: Role::Mid }, 150);
    transition_penalty.insert(PenaltyRolePair { main:Role::Sup, assign: Role::Jg }, 150);
    transition_penalty.insert(PenaltyRolePair { main:Role::Adc, assign: Role::Sup }, 80);
    transition_penalty.insert(PenaltyRolePair { main:Role::Jg, assign: Role::Top }, 110);
    transition_penalty.insert(PenaltyRolePair { main:Role::Jg, assign: Role::Mid }, 110);
    transition_penalty.insert(PenaltyRolePair { main:Role::Jg, assign: Role::Adc }, 110);


    Self {
      sub_role_penalty: 50,
      off_role_penalty: 125,
      role_transition_penalty_map: transition_penalty,
    }
  }
}

impl PenaltyMatrix {
  pub fn pref_penalty(&self, p: &Player, assigned: Role) -> i32 {
    if assigned == p.main_role {
      0
    } else if p.sub_role.contains(&assigned)  {
      self.sub_role_penalty
    } else {
      self.off_role_penalty
    }
  }

  pub fn transfer_extra(&self, from_main: Role, to_role: Role) -> i32 {
    *self.role_transition_penalty_map.get(&PenaltyRolePair {
      main: from_main.clone(),
      assign: to_role.clone(),
    }).unwrap_or(&0)
  }

  pub fn total_penalty(&self, p: &Player, assigned: Role) -> i32 {
    let base = self.pref_penalty(p, assigned.clone());
    let extra = self.transfer_extra(p.main_role.clone(), assigned.clone());
    base + extra
  }
}