use rayon::iter::ParallelIterator;

use crate::adapter::graph::Graph;
use crate::domain::types::{ExecutionParams, Route, Side, SwapStep, Token};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

/// Finds the best route from `from` to `to` using a breadth-first search (BFS) algorithm.
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
pub fn bfs(
    _side: Side,
    graph: &Graph,
    from: &Token,
    to: &Token,
    amount_in: f64,
    params: ExecutionParams,
) -> Option<Route> {
    let mut queue = VecDeque::new();
    queue.push_back((from.clone(), vec![], amount_in));

    let mut visited: HashMap<Token, f64> = HashMap::new();

    while let Some((token, route, cumulative_amount)) = queue.pop_front() {
        if token == *to {
            return Some(Route {
                steps: route,
                output_amount: cumulative_amount,
            });
        }
        if route.len() >= params.max_hops {
            continue;
        }
        if let Some(&seen_amt) = visited.get(&token) {
            if seen_amt >= cumulative_amount {
                continue;
            }
        }
        visited.insert(token.clone(), cumulative_amount);

        for (next_token, pool) in graph.neighbors(&token).collect::<Vec<_>>() {
            if let Some(out) = pool.get_output_amount(&token, cumulative_amount) {
                let mut new_route = route.clone();
                new_route.push(SwapStep {
                    from: token.clone(),
                    to: next_token.clone(),
                    pool: Arc::clone(&pool),
                });
                queue.push_back((next_token.clone(), new_route, out));
            }
        }
    }

    None
}
