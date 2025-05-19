use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rust_decimal::dec;

use crate::adapter::graph::Graph;
use crate::domain::{self, types::*};
use crate::port::routing::{DefaultRouter, RoutingStrategy};
use std::sync::Arc;

fn get_reference_pools() -> Vec<Arc<Pool>> {
    vec![
        Pool {
            token0: Token("ETH"),
            token1: Token("USDC"),
            reserve0: dec!(2000),
            reserve1: dec!(2000000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("USDC"),
            reserve0: dec!(1000),
            reserve1: dec!(1000000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("DAI"),
            reserve0: dec!(1000),
            reserve1: dec!(900000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("DAI"),
            reserve0: dec!(3000),
            reserve1: dec!(2800000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("DAI"),
            reserve0: dec!(3000),
            reserve1: dec!(3100000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("USDC"),
            reserve0: dec!(1000000),
            reserve1: dec!(1000000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("USDC"),
            reserve0: dec!(2000000),
            reserve1: dec!(2000000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("USDT"),
            reserve0: dec!(1000000),
            reserve1: dec!(900000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("DAI"),
            token1: Token("USDT"),
            reserve0: dec!(900000),
            reserve1: dec!(1000000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("USDT"),
            reserve0: dec!(2000),
            reserve1: dec!(2000000),
            fee_bps: dec!(30),
        },
        Pool {
            token0: Token("ETH"),
            token1: Token("USDT"),
            reserve0: dec!(10000),
            reserve1: dec!(10000000),
            fee_bps: dec!(30),
        },
    ]
    .into_par_iter()
    .map(Arc::new)
    .collect()
}

fn validate_route(route: (RoutingAlgo, Route), from: &'static str, to: &'static str) {
    let (_algo, route) = route;
    assert!(route.output_amount > dec!(0), "Output amount should be > 0");
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

    let eth_in = dec!(10);
    let usdc_in = dec!(10000);

    let route1 = router.compute_route(
        Side::Buy,
        &graph,
        &Token("ETH"),
        &Token("USDC"),
        eth_in,
        ExecutionParams {
            slippage: Slippage {
                tolerance_bps: dec!(100),
            },
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
            slippage: Slippage {
                tolerance_bps: dec!(100),
            },
            algo: RoutingAlgo::Auto,
            max_hops: 4,
        },
    );
    assert!(route2.is_some(), "USDC → ETH route not found");
    validate_route(route2.unwrap(), "USDC", "ETH");
}
