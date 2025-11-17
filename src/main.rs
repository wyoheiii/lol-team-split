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

    let eval = EvalContext::new(&lobby);
    let evaluator = Evaluator::new(eval);
    let optimizer = JointEnumeratingOptimizer::new(evaluator, Some(42));
    let solver   = DefaultSolver::new(optimizer);
    print_teams(&solver.solve(&lobby) , &lobby, &id);
  }

}
