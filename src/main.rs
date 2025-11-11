use crate::{domain::Rank, domain::states::Lobby};
use crate::solver::TeamSolver;
use crate::splitter::random::RandomSplitter;
use crate::assigner::random::RandomRoleAssigner;
use crate::solver::default::DefaultSolver;
use print::console::print_teams;
use demo_lobby::sample_lobbies;



mod domain;
mod param;
mod pipeline;
mod splitter;
mod assigner;
mod print;
mod solver;
mod demo_lobby;



fn main() {

  for (id, lobby) in sample_lobbies() {
    println!("=== Solving Lobby: {} ===", id);
    let splitter = RandomSplitter::new(42);
    let assigner = RandomRoleAssigner::new(42);
    let solver   = DefaultSolver::new(splitter, assigner);
    print_teams(&solver.solve(&lobby));
  }

}
