use std::cmp::Ordering;
use std::collections::BinaryHeap;
use rayon::iter::ParallelIterator;
use rust_decimal::Decimal;
use std::sync::Arc;
use crate::domain::types::{Side, Token, SwapStep, Route, ExecutionParams};
use crate::adapter::graph::Graph;


#[derive(Debug, Clone)]
struct CircularState {
    token: Token,
    route: Vec<SwapStep>,
    cumulative_amount: Decimal,
}

impl PartialEq for CircularState {
    fn eq(&self, other: &Self) -> bool {
        self.cumulative_amount.eq(&other.cumulative_amount)
    }
}
impl Eq for CircularState {}
impl PartialOrd for CircularState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CircularState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cumulative_amount.cmp(&self.cumulative_amount)
    }
}


pub fn circular(
    _side: Side,
    graph: &Graph,
    from: &Token,
    _to: &Token, // not used, as we want cycles starting and ending at `from`
    amount_in: Decimal,
    params: ExecutionParams,
) -> Option<Route> {
    let mut heap = BinaryHeap::new();
    heap.push(CircularState {
        token: from.clone(),
        route: vec![],
        cumulative_amount: amount_in,
    });

    let mut best_route: Option<Route> = None;

    while let Some(CircularState {
        token,
        route,
        cumulative_amount,
    }) = heap.pop()
    {
        // Only consider cycles that return to the starting token, and are not empty
        if token == *from && !route.is_empty() {
            let candidate = Route {
                steps: route.clone(),
                output_amount: cumulative_amount,
            };
            if best_route
                .as_ref()
                .map_or(true, |r| candidate.output_amount > r.output_amount)
            {
                best_route = Some(candidate);
            }
            // Do not continue from here, as we don't want to extend cycles further
            continue;
        }
        if route.len() >= params.max_hops {
            continue;
        }

        for (next_token, pool) in graph.neighbors(&token).collect::<Vec<_>>() {
            // Prevent revisiting tokens in the same route (except for returning to `from`)
            if route.iter().any(|step| step.to == next_token) && next_token != *from {
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

            heap.push(CircularState {
                token: next_token.clone(),
                route: new_route,
                cumulative_amount: out,
            });
        }
    }

    best_route
}
