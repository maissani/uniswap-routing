use rust_decimal::dec;
use std::sync::Arc;
use uniswap_routing::adapter::graph::Graph;
use uniswap_routing::application::algos::best_direct::best_direct;
use uniswap_routing::domain::types::*;
use uniswap_routing::infra::algo_selector::select_best_algo;
use uniswap_routing::port::routing::{DefaultRouter, RoutingStrategy};

fn get_demo_pools() -> Vec<Arc<Pool>> {
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
    .into_iter()
    .map(Arc::new)
    .collect()
}

fn main() {
    let pools = get_demo_pools();
    let graph = Graph::new(&pools);
    let router = DefaultRouter;

    let scenarios = vec![
        (Side::Buy, "ETH", "USDC", dec!(10), dec!(100)), // Scenario that is expecter on the solve1
        (Side::Buy, "USDC", "ETH", dec!(10000), dec!(100)), // Scenario that is expected on the solve2
        (Side::Buy, "USDT", "ETH", dec!(20000), dec!(30)),
        (Side::Buy, "ETH", "USDT", dec!(5), dec!(50)),
        (Side::Buy, "ETH", "USDC", dec!(10), dec!(100)),
        (Side::Buy, "USDC", "ETH", dec!(10000), dec!(100)),
        (Side::Buy, "ETH", "DAI", dec!(2.5), dec!(150)),
        (Side::Buy, "DAI", "ETH", dec!(2.5), dec!(150)),
        (Side::Buy, "USDC", "DAI", dec!(10000), dec!(100)),
        (Side::Buy, "DAI", "USDC", dec!(10000), dec!(100)),
        (Side::Buy, "USDC", "USDT", dec!(10000), dec!(100)),
        (Side::Buy, "USDT", "USDC", dec!(10000), dec!(100)),
        (Side::Buy, "DAI", "USDT", dec!(10000), dec!(100)),
        (Side::Buy, "USDT", "DAI", dec!(10000), dec!(100)),
        (Side::Buy, "USDC", "ETH", dec!(10000), dec!(100)),
        (Side::Buy, "ETH", "USDC", dec!(10000), dec!(100)),
        (Side::Buy, "USDT", "ETH", dec!(10000), dec!(100)),
        (Side::Buy, "ETH", "USDT", dec!(10000), dec!(100)),
    ];

    for (side, from, to, input, slippage_bps) in scenarios {

        let slippage = Slippage {
            tolerance_bps: slippage_bps,
        };
        let algo = select_best_algo(graph.tokens().len(), pools.len(), slippage);

        println!(
            "\n=== Scenario: {} → {} | Input: {} | Slippage: {}bps | Algo: {:?} ===",
            from, to, input, slippage_bps, algo
        );

        // Detect and print arbitrage opportunities (circular routes)
        let circular_routes = uniswap_routing::application::algos::circular::circular(
            side.clone(),
            &graph,
            &Token(from),
            &Token(to), // not used, but required by signature
            input,
            ExecutionParams {
            slippage,
            algo,
            max_hops: 4,
            },
        );

        for route in circular_routes.iter() {
            if route.output_amount > input {
            println!(
                "→ Arbitrage Type Circular: possible!  Meilleur Profit: Route: {} | Profit: {:.6}",
                route
                .steps
                .iter()
                .map(|step| format!("{}→{}", step.from.0, step.to.0))
                .collect::<Vec<_>>()
                .join(" -> "),
                route.output_amount - input
            );
            }
        }

        if let Some((algo, r)) = router.compute_route(
            side.clone(),
            &graph,
            &Token(from),
            &Token(to),
            input,
            ExecutionParams {
                slippage,
                algo,
                max_hops: 4,
            },
        ) {
            if side == Side::Buy {
                print!("→ SIDE BUY:");
            } else {
                print!("→ SIDE SELL:");
            }
            println!("→ Best route algo: {:?}", algo);
            if let Some(best_direct) = best_direct(
                side.clone(),
                &graph,
                &Token(from),
                &Token(to),
                input,
                ExecutionParams {
                    algo,
                    slippage,
                    max_hops: 4,
                },
            ) {
                println!("→ Best direct: {:.6}", best_direct.output_amount);
                println!("→ Best Route Output: {:.6}", r.output_amount);
                if best_direct.output_amount > dec!(0) {
                    let improvement = ((r.output_amount - best_direct.output_amount)
                        / best_direct.output_amount)
                        * dec!(100);
                    println!("→ Improvement over best direct swap: {:.2}%", improvement);
                }
                println!("→ No direct swap available");
            }

            println!("→ Steps: {} hops", r.steps.len());
            for step in r.steps.iter() {
                println!(
                    "  {} → {} via [{} / {}]",
                    step.from.0, step.to.0, step.pool.token0.0, step.pool.token1.0
                );
            }
        } else {
            println!("No route found for {} → {}", from, to);
        };
    }
}
