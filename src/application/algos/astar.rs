use rayon::iter::ParallelIterator;
use rust_decimal::{Decimal, dec};

use crate::adapter::graph::Graph;
use crate::domain::types::{ExecutionParams, Route, Side, SwapStep, Token};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct AStarState {
    token: Token,
    route: Vec<SwapStep>,
    cumulative_amount: Decimal,
    estimated_cost: Decimal,
    visited_tokens: HashSet<Token>,
}

impl PartialEq for AStarState {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost)
    }
}
impl Eq for AStarState {}
impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

/// A* algorithm for finding the best route
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
pub fn astar(
    _side: Side,
    graph: &Graph,
    from: &Token,
    to: &Token,
    amount_in: Decimal,
    params: ExecutionParams,
) -> Option<Route> {
    let mut heap = BinaryHeap::new();
    let mut initial_seen = HashSet::new();
    initial_seen.insert(from.clone());

    heap.push(AStarState {
        token: from.clone(),
        route: vec![],
        cumulative_amount: amount_in,
        estimated_cost: dec!(0),
        visited_tokens: initial_seen,
    });

    let mut visited: HashMap<Token, Decimal> = HashMap::new();

    while let Some(AStarState {
        token,
        route,
        cumulative_amount,
        visited_tokens,
        ..
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
            if visited_tokens.contains(&next_token) {
                continue;
            }
            let Some(out) = pool.get_output_amount(&token, cumulative_amount) else {
                continue;
            };

            let mut new_route = route.clone();
            new_route.push(SwapStep {
                from: token.clone(),
                to: next_token.clone(),
                pool: Arc::clone(&pool),
            });

            let mut new_visited = visited_tokens.clone();
            new_visited.insert(next_token.clone());

            let liquidity_heuristic = dec!(1) / (pool.reserve0 + pool.reserve1).max(dec!(1));
            let estimated_cost = -out + liquidity_heuristic;

            heap.push(AStarState {
                token: next_token.clone(),
                route: new_route,
                cumulative_amount: out,
                estimated_cost,
                visited_tokens: new_visited,
            });
        }
    }

    None
}
