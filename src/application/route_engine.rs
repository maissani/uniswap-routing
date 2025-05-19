use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::adapter::graph::Graph;
use crate::application::algos::{
    astar::astar, best_direct::best_direct, bfs::bfs, dfs::dfs, dijkstra::dijkstra,
};
use crate::domain::types::{ExecutionParams, Route, RoutingAlgo, Side, Token};

pub fn execute(
    side: Side,
    graph: &Graph,
    from: &Token,
    to: &Token,
    amount_in: f64,
    exec_params: ExecutionParams,
) -> Option<(RoutingAlgo, Route)> {
    match exec_params.algo {
        RoutingAlgo::BestDirect => best_direct(side, graph, from, to, amount_in, exec_params)
            .map(|route| (RoutingAlgo::BestDirect, route)),
        RoutingAlgo::AStar => astar(side, graph, from, to, amount_in, exec_params)
            .map(|route| (RoutingAlgo::AStar, route)),
        RoutingAlgo::Dijkstra => dijkstra(side, graph, from, to, amount_in, exec_params)
            .map(|route| (RoutingAlgo::Dijkstra, route)),
        RoutingAlgo::Bfs => bfs(side, graph, from, to, amount_in, exec_params)
            .map(|route| (RoutingAlgo::Bfs, route)),
        RoutingAlgo::Dfs => dfs(side, graph, from, to, amount_in, exec_params)
            .map(|route| (RoutingAlgo::Dfs, route)),
        RoutingAlgo::Auto => select_best_route(side, graph, from, to, amount_in, exec_params),
    }
}

pub fn select_best_route(
    side: Side,
    graph: &Graph,
    from: &Token,
    to: &Token,
    amount_in: f64,
    params: ExecutionParams,
) -> Option<(RoutingAlgo, Route)> {
    let candidates = [
        (
            RoutingAlgo::BestDirect,
            best_direct(side.clone(), graph, from, to, amount_in, params),
        ),
        (
            RoutingAlgo::AStar,
            astar(side.clone(), graph, from, to, amount_in, params),
        ),
        (
            RoutingAlgo::Dijkstra,
            dijkstra(side.clone(), graph, from, to, amount_in, params),
        ),
        (
            RoutingAlgo::Bfs,
            bfs(side.clone(), graph, from, to, amount_in, params),
        ),
        (
            RoutingAlgo::Dfs,
            dfs(side.clone(), graph, from, to, amount_in, params),
        ),
    ];

    candidates
        .into_par_iter()
        .filter_map(|(algo, opt)| opt.map(|r| (algo, r)))
        .max_by(|a, b| a.1.output_amount.total_cmp(&b.1.output_amount))
}
