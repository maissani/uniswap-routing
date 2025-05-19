use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::adapter::graph::Graph;
use crate::domain::{self, types::*};
use crate::port::routing::{DefaultRouter, RoutingStrategy};
use std::sync::Arc;

fn get_reference_pools() -> Vec<Arc<Pool>> {
    vec![
        Pool {
            token0: Token("ETH"),
            token1: Token("USDC"),
            reserve0: 2000.0,
            reserve1: 2_000_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("USDC"),
            reserve0: 1000.0,
            reserve1: 1_000_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("DAI"),
            reserve0: 1000.0,
            reserve1: 900_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("DAI"),
            reserve0: 3000.0,
            reserve1: 2_800_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("DAI"),
            reserve0: 3000.0,
            reserve1: 3_100_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("USDC"),
            reserve0: 1_000_000.0,
            reserve1: 1_000_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("USDC"),
            reserve0: 2_000_000.0,
            reserve1: 2_000_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("USDT"),
            reserve0: 1_000_000.0,
            reserve1: 900_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("USDT"),
            reserve0: 900_000.0,
            reserve1: 1_000_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("USDT"),
            reserve0: 2000.0,
            reserve1: 2_000_000.0,
            fee_bps: 30,
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("USDT"),
            reserve0: 10_000.0,
            reserve1: 10_000_000.0,
            fee_bps: 30,
        },
    ]
    .into_par_iter()
    .map(Arc::new)
    .collect()
}

fn validate_route(route: (RoutingAlgo, Route), from: &'static str, to: &'static str) {
    let (_algo, route) = route;
    assert!(route.output_amount > 0.0, "Output amount should be > 0");
    assert_eq!(
        route.steps.first().unwrap().from,
        domain::types::Token(from)
    );
    assert_eq!(route.steps.last().unwrap().to, domain::types::Token(to));

    for i in 1..route.steps.len() {
        assert_eq!(route.steps[i - 1].to, route.steps[i].from);
    }
}

#[test]
fn test_full_eth_usdc_routing_scenario() {
    let pools = get_reference_pools();
    let graph = Graph::new(&pools);
    let router = DefaultRouter;

    let eth_in = 10.0;
    let usdc_in = 10_000.0;

    let route1 = router.compute_route(
        Side::Buy,
        &graph,
        &Token("ETH"),
        &Token("USDC"),
        eth_in,
        ExecutionParams {
            slippage: Slippage { tolerance_bps: 100 },
            algo: RoutingAlgo::Auto,
            max_hops: 4,
        },
    );
    assert!(route1.is_some(), "ETH → USDC route not found");
    validate_route(route1.unwrap(), "ETH", "USDC");

    let route2 = router.compute_route(
        Side::Buy,
        &graph,
        &Token("USDC"),
        &Token("ETH"),
        usdc_in,
        ExecutionParams {
            slippage: Slippage { tolerance_bps: 100 },
            algo: RoutingAlgo::Auto,
            max_hops: 4,
        },
    );
    assert!(route2.is_some(), "USDC → ETH route not found");
    validate_route(route2.unwrap(), "USDC", "ETH");
}
