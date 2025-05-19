use rayon::iter::ParallelIterator;

use crate::adapter::graph::Graph;
use crate::domain::types::{ExecutionParams, Route, Side, SwapStep, Token};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct DijkstraState {
    token: Token,
    route: Vec<SwapStep>,
    cumulative_amount: f64,
}

impl PartialEq for DijkstraState {
    fn eq(&self, other: &Self) -> bool {
        self.cumulative_amount.eq(&other.cumulative_amount)
    }
}
impl Eq for DijkstraState {}
impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cumulative_amount.total_cmp(&self.cumulative_amount)
    }
}

/// Dijkstra's algorithm for finding the best route
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
pub fn dijkstra(
    _side: Side,
    graph: &Graph,
    from: &Token,
    to: &Token,
    amount_in: f64,
    params: ExecutionParams,
) -> Option<Route> {
    let mut heap = BinaryHeap::new();
    heap.push(DijkstraState {
        token: from.clone(),
        route: vec![],
        cumulative_amount: amount_in,
    });

    let mut visited: HashMap<Token, f64> = HashMap::new();

    while let Some(DijkstraState {
        token,
        route,
        cumulative_amount,
    }) = heap.pop()
    {
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
            let Some(out) = pool.get_output_amount(&token, cumulative_amount) else {
                continue;
            };
            let mut new_route = route.clone();
            new_route.push(SwapStep {
                from: token.clone(),
                to: next_token.clone(),
                pool: Arc::clone(&pool),
            });

            heap.push(DijkstraState {
                token: next_token.clone(),
                route: new_route,
                cumulative_amount: out,
            });
        }
    }

    None
}
