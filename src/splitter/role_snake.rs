use std::collections::HashMap;

use crate::{domain::{states::{Lobby, SplitTeams}, Player, Role}, param::mmr::MMR, pipeline};


#[derive(Debug, Clone)]
pub struct RoleSnakeSplitter{ pub mmr: MMR }

impl RoleSnakeSplitter {
  fn sort_by_main_role(&self, lobby: &Lobby) -> HashMap<Role, Vec<Player>> {
    let mut by_role: HashMap<Role, Vec<Player>> = HashMap::new();

    for r in &Role::All { by_role.insert(r.clone(), Vec::new()); }

    for p in lobby.players() {
      by_role.get_mut(&p.main_role).unwrap().push(p.clone());
    }

    for v in by_role.values_mut() {
      v.sort_by(|a, b| {
        let mmr_a = self.mmr.calculate(&a.rank);
        let mmr_b = self.mmr.calculate(&b.rank);
        mmr_b.partial_cmp(&mmr_a).unwrap()
      });
    }

    by_role
  }
}

impl pipeline::TeamSplitter for RoleSnakeSplitter {
  fn split(&self, lobby: &Lobby) -> SplitTeams {
    let mut by_role = self.sort_by_main_role(lobby);
    let mut red = Vec::with_capacity(5);
    let mut blue = Vec::with_capacity(5);


    for r in &Role::All {
      let mut group = by_role.remove(&r).unwrap_or_default();
      let i = 0usize;
      let mut toggle = true; // snake

      while i < group.len() {
        if red.len() == 5 {
          blue.push(group.remove(i));
          continue;
        }
        if blue.len() == 5 {
          red.push(group.remove(i));
          continue;
        }
        if toggle {
          red.push(group.remove(i));
        } else {
          blue.push(group.remove(i));
        }
        toggle = !toggle;
      }
    }

    SplitTeams::new(red.try_into().unwrap(), blue.try_into().unwrap())
  }
}