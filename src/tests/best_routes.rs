use crate::adapter::graph::Graph;
use crate::domain::types::*;
use crate::infra::algo_selector::select_best_algo;
use crate::port::routing::{DefaultRouter, RoutingStrategy};
use std::sync::Arc;
use std::io::{self, Write};

fn get_reference_pools() -> Vec<Arc<Pool>> {
    vec![
        Pool { token0: Token("ETH"), token1: Token("USDC"), reserve0: 2000.0, reserve1: 2_000_000.0, fee_bps: 30 },
        Pool { token0: Token("ETH"), token1: Token("USDC"), reserve0: 1000.0, reserve1: 1_000_000.0, fee_bps: 30 },
        Pool { token0: Token("ETH"), token1: Token("DAI"), reserve0: 1000.0, reserve1: 900_000.0, fee_bps: 30 },
        Pool { token0: Token("ETH"), token1: Token("DAI"), reserve0: 3000.0, reserve1: 2_800_000.0, fee_bps: 30 },
        Pool { token0: Token("ETH"), token1: Token("DAI"), reserve0: 3000.0, reserve1: 3_100_000.0, fee_bps: 30 },
        Pool { token0: Token("DAI"), token1: Token("USDC"), reserve0: 1_000_000.0, reserve1: 1_000_000.0, fee_bps: 30 },
        Pool { token0: Token("DAI"), token1: Token("USDC"), reserve0: 2_000_000.0, reserve1: 2_000_000.0, fee_bps: 30 },
        Pool { token0: Token("DAI"), token1: Token("USDT"), reserve0: 1_000_000.0, reserve1: 900_000.0, fee_bps: 30 },
        Pool { token0: Token("DAI"), token1: Token("USDT"), reserve0: 900_000.0, reserve1: 1_000_000.0, fee_bps: 30 },
        Pool { token0: Token("ETH"), token1: Token("USDT"), reserve0: 2000.0, reserve1: 2_000_000.0, fee_bps: 30 },
        Pool { token0: Token("ETH"), token1: Token("USDT"), reserve0: 10_000.0, reserve1: 10_000_000.0, fee_bps: 30 },
    ]
    .into_par_iter()
    .map(Arc::new)
    .collect()
}

fn validate_route(route: &Route, from: &'static str, to: &'static str) {
    assert!(route.output_amount > 0.0, "Output amount should be > 0");
    assert_eq!(route.steps.first().unwrap().from, from);
    assert_eq!(route.steps.last().unwrap().to, to);

    for i in 1..route.steps.len() {
        assert_eq!(route.steps[i - 1].to, route.steps[i].from);
    }
}

#[test]
fn test_multiple_best_routes_and_algo_selection() {
    let pools = get_reference_pools();
    let graph = Graph::new(&pools);
    let router = DefaultRouter;
    let scenarios = vec![
        ("ETH", "USDC", 10.0, 100),
        ("USDC", "ETH", 10_000.0, 100),
        ("ETH", "USDT", 5.0, 50),
        ("USDT", "ETH", 20_000.0, 30),
        ("ETH", "DAI", 2.5, 150),
    ];

    for (from, to, input, slippage_bps) in scenarios {
        let slippage = Slippage { tolerance_bps: slippage_bps };
        let algo = select_best_algo(graph.tokens().len(), pools.len(), slippage);

        println!("\nScenario: {} → {} | Input: {} | Slippage: {}bps | Algo: {:?}", from, to, input, slippage_bps, algo);

        let route = router.compute_route(
            &graph,
            &Token(from),
            &Token(to),
            input,
            ExecutionParams { slippage, algo },
        );

        assert!(route.is_some(), "No route found for {} → {}", from, to);
        let route = route.unwrap();
        validate_route(&route, from, to);
        println!("Best route output: {:.6} | Steps: {}", route.output_amount, route.steps.len());
    }
}
