use rayon::iter::ParallelIterator;
use rust_decimal::Decimal;

use crate::adapter::graph::Graph;
use crate::domain::types::{ExecutionParams, Route, Side, SwapStep, Token};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

/// Finds the best route from `from` to `to` using a depth-first search (DFS) algorithm.
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
pub fn dfs(
    _side: Side,
    graph: &Graph,
    from: &Token,
    to: &Token,
    amount_in: Decimal,
    params: ExecutionParams,
) -> Option<Route> {
    let mut stack = VecDeque::new();
    stack.push_back((
        from.clone(),
        vec![],
        amount_in,
        HashSet::from([from.clone()]),
    ));

    let mut visited: HashMap<Token, Decimal> = HashMap::new();

    while let Some((token, route, cumulative_amount, seen)) = stack.pop_back() {
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
            if seen.contains(&next_token) {
                continue;
            }
            if let Some(out) = pool.get_output_amount(&token, cumulative_amount) {
                let mut new_route = route.clone();
                new_route.push(SwapStep {
                    from: token.clone(),
                    to: next_token.clone(),
                    pool: Arc::clone(&pool),
                });
                let mut new_seen = seen.clone();
                new_seen.insert(next_token.clone());
                stack.push_back((next_token.clone(), new_route, out, new_seen));
            }
        }
    }

    None
}
