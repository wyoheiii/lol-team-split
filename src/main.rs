use crate::engine::evaluator::Evaluator;
use crate::optimizer::joint::JointEnumeratingOptimizer;
use crate::param::eval::EvalContext;
use crate::pipeline::DefaultSolver;
use print::console::print_teams;
use demo_lobby::sample_lobbies;



mod domain;
mod param;
mod pipeline;
mod print;
mod demo_lobby;
mod engine;
mod optimizer;


fn main() {

  for (id, lobby) in sample_lobbies() {
    println!("=== Solving Lobby: {} ===", id);
    for p in lobby.players() {
      println!("  - {:<12} (Main Role: {} , Sub Role: {:?} , Rank: {})", p.name, p.main_role, p.sub_role, p.rank);
    }

    let eval = EvalContext::new(&lobby);
    let evaluator = Evaluator::new(eval);
    let optimizer = JointEnumeratingOptimizer::new(evaluator, Some(42));
    let solver   = DefaultSolver::new(optimizer);
    print_teams(&solver.solve(&lobby));
  }

}
