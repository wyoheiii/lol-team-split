use crate::domain::Player;
use crate::domain::states::{AssignedTeams, Lobby};
use crate::domain::types::{RoleMap, Side};

pub fn print_teams(teams: &AssignedTeams, lobby: &Lobby , id: &str) {

  println!("=== Solving Lobby: {} ===", id);
  for p in lobby.players() {
    println!("  - {:<12} (Main Role: {} , Sub Role: {:?} , Rank: {})", p.name, p.main_role, p.sub_role, p.rank);
  }

  println!("========= TEAM POWER =========");
  println!("  - main: {}, sub: {}, off: {}", teams.red.main, teams.red.sub, teams.red.off);
  println!("  - Red Team Power  : {:.2}", teams.red.power);
  println!("  - main: {}, sub: {}, off: {}", teams.blue.main, teams.blue.sub, teams.blue.off);
  println!("  - Blue Team Power : {:.2}", teams.blue.power);
  println!("  - Difference      : {:.2}", (teams.red.power - teams.blue.power).abs());
  println!("");

  let p = |t :&RoleMap<Player>, side: Side| {
    println!("========= {} SIDE =========", side);
    println!("{:<12}  {:<4}  {}", "Player", "Role", "Rank");
    println!("-------------------------------------");
    for (p, r) in t.iter() {
      println!("{:<12}  {:<4}  {}", p.name, r, p.rank);
    }
    println!();
  };

  p(&teams.red.players, Side::Red);
  p(&teams.blue.players, Side::Blue);
}