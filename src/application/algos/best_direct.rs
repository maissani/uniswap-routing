use rayon::iter::ParallelIterator;

use crate::adapter::graph::Graph;
use crate::domain::types::{ExecutionParams, Route, Side, SwapStep, Token};
use std::sync::Arc;

/// Finds the best direct route from `from` to `to` using the provided `graph`.
///
/// Args:
/// - `_side`: The side of the trade (not used in this implementation).
/// - `graph`: The graph representing the pools.
/// - `from`: The starting token.
/// - `to`: The target token.
/// - `amount_in`: The amount of the starting token.
/// - `params`: Execution parameters, including max hops.
///
/// Returns:
/// - An `Option<Route>` containing the best route if found, or `None` if no route exists.
pub fn best_direct(
    _side: Side,
    graph: &Graph,
    from: &Token,
    to: &Token,
    amount_in: f64,
    _params: ExecutionParams,
) -> Option<Route> {
    graph
        .neighbors(from)
        .filter(|(t, _)| t == to)
        .filter_map(|(_, pool)| {
            pool.get_output_amount(from, amount_in).map(|out| Route {
                steps: vec![SwapStep {
                    from: from.clone(),
                    to: to.clone(),
                    pool: Arc::clone(&pool),
                }],
                output_amount: out,
            })
        })
        .max_by(|a, b| a.output_amount.total_cmp(&b.output_amount))
}
