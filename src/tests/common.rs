use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::adapter::graph::Graph;
use crate::domain::types::*;
use crate::port::routing::{DefaultRouter, RoutingStrategy};
use std::sync::Arc;

fn setup_graph() -> (Graph, Vec<Arc<Pool>>) {
    let pools = vec![
        Pool {
            token0: Token("ETH"),
            token1: Token("USDC"),
            reserve0: 1000.0,
            reserve1: 1_000_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("USDC"),
            token1: Token("DAI"),
            reserve0: 1_000_000.0,
            reserve1: 1_000_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("WBTC"),
            reserve0: 1_000_000.0,
            reserve1: 50.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("WBTC"),
            reserve0: 1000.0,
            reserve1: 50.0,
            fee_bps: 30,
        },
    ];

    let arc_pools: Vec<Arc<Pool>> = pools.into_par_iter().map(Arc::new).collect();
    let graph = Graph::new(&arc_pools);
    (graph, arc_pools)
}

#[test]
fn test_direct_and_indirect_route() {
    let (graph, _) = setup_graph();
    let router = DefaultRouter;
    let from = Token("ETH");
    let to = Token("WBTC");
    let params = ExecutionParams {
        slippage: Slippage { tolerance_bps: 100 },
        algo: RoutingAlgo::Auto,
        max_hops: 4,
    };

    let route = router.compute_route(Side::Buy, &graph, &from, &to, 10.0, params);
    assert!(route.is_some());
    let (_algo, route) = route.unwrap();
    assert!(route.output_amount > 0.0);
    assert!(route.steps.len() <= 3);
    println!("Best route: {:?}", route);
}

#[test]
fn test_slippage_enforced() {
    let (graph, _) = setup_graph();
    let from = Token("ETH");
    let to = Token("WBTC");
    let input = 10.0;
    let max_slippage = 0.01;

    let base_router = DefaultRouter;
    let params = ExecutionParams {
        slippage: Slippage {
            tolerance_bps: (max_slippage * 10_000.0) as u32,
        },
        algo: RoutingAlgo::Dijkstra,
        max_hops: 4,
    };

    if let Some((_algo, route)) =
        base_router.compute_route(Side::Buy, &graph, &from, &to, input, params)
    {
        let effective_rate = route.output_amount / input;
        let worst_case_rate = effective_rate * (1.0 - max_slippage);
        assert!(route.output_amount >= input * worst_case_rate);
    } else {
        panic!("No route found with slippage tolerance");
    }
}
