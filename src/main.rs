use crate::param::eval::EvalContext;
use crate::splitter::role_snake::RoleSnakeSplitter;
use crate::{domain::Rank, domain::states::Lobby};
use crate::pipeline::TeamSolver;
use crate::splitter::random::RandomSplitter;
use crate::assigner::random::RandomRoleAssigner;
use crate::assigner::brute_force::BruteForceAssigner;
use crate::pipeline::DefaultSolver;
use print::console::print_teams;
use demo_lobby::sample_lobbies;



mod domain;
mod param;
mod pipeline;
mod splitter;
mod assigner;
mod print;
mod demo_lobby;



fn main() {

  for (id, lobby) in sample_lobbies() {
    println!("=== Solving Lobby: {} ===", id);
    for p in lobby.players() {
      println!("  - {:<12} (Role: {} ,Rank: {})", p.name, p.main_role, p.rank);
    }
    // let splitter = RandomSplitter::new(42);
    // let assigner = RandomRoleAssigner::new(42);
    let eval = EvalContext::new(&lobby);
    let splitter = RoleSnakeSplitter::new(eval.eval.mmr.clone());
    let assigner = BruteForceAssigner::new(eval);
    let solver   = DefaultSolver::new(splitter, assigner);
    print_teams(&solver.solve(&lobby));
  }

}
