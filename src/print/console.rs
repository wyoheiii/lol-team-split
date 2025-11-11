use crate::domain::Player;
use crate::domain::states::AssignedTeams;
use crate::domain::types::{RoleMap, Side};

pub fn print_teams(teams: &AssignedTeams) {
  let p = |t :&RoleMap<Player>, side: Side| {
    println!("========= {} SIDE =========", side);
    println!("{:<12}  {:<4}  {}", "Player", "Role", "Rank");
    println!("-------------------------------------");
    for (p, r) in t.iter() {
      println!("{:<12}  {:<4}  {}", p.name, r, p.rank);
    }
    println!();
  };

  p(&teams.red, Side::Red);
  p(&teams.blue, Side::Blue);
}