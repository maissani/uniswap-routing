use crate::domain::types::{RoutingAlgo, Slippage};

pub fn select_best_algo(_num_tokens: usize, _num_pools: usize, _slippage: Slippage) -> RoutingAlgo {
    // let avg_degree = if num_tokens > 0 {
    //     num_pools as f64 / num_tokens as f64
    // } else {
    //     0.0
    // };

    // let slippage_factor = slippage.tolerance_bps;

    // match (num_tokens, avg_degree, slippage_factor) {
    //     (n, d, s) if n <= 10 && d < 3.0 && s >= 100 => RoutingAlgo::Bfs,
    //     (n, d, s) if n <= 20 && d <= 5.0 && s >= 50 => RoutingAlgo::Dfs,
    //     (n, _, s) if n <= 50 && s >= 25 && s <= 100 => RoutingAlgo::Dijkstra,
    //     _ => RoutingAlgo::AStar,
    // }
    RoutingAlgo::Auto
}
