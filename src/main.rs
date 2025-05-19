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
    .into_iter()
    .map(Arc::new)
    .collect()
}

fn main() {
    let pools = get_demo_pools();
    let graph = Graph::new(&pools);
    let router = DefaultRouter;

    let scenarios = vec![
        (Side::Buy, "ETH", "USDC", 10.0, 100), // Scenario that is expecter on the solve1
        (Side::Buy, "USDC", "ETH", 10_000.0, 100), // Scenario that is expected on the solve2
        (Side::Buy, "USDT", "ETH", 20_000.0, 30),
        (Side::Buy, "ETH", "USDT", 5.0, 50),
        (Side::Buy, "ETH", "USDC", 10.0, 100),
        (Side::Buy, "USDC", "ETH", 10_000.0, 100),
        (Side::Buy, "ETH", "DAI", 2.5, 150),
        (Side::Buy, "DAI", "ETH", 2.5, 150),
        (Side::Buy, "USDC", "DAI", 10_000.0, 100),
        (Side::Buy, "DAI", "USDC", 10_000.0, 100),
        (Side::Buy, "USDC", "USDT", 10_000.0, 100),
        (Side::Buy, "USDT", "USDC", 10_000.0, 100),
        (Side::Buy, "DAI", "USDT", 10_000.0, 100),
        (Side::Buy, "USDT", "DAI", 10_000.0, 100),
        (Side::Buy, "USDC", "ETH", 10_000.0, 100),
        (Side::Buy, "ETH", "USDC", 10_000.0, 100),
        (Side::Buy, "USDT", "ETH", 10_000.0, 100),
        (Side::Buy, "ETH", "USDT", 10_000.0, 100),
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
                if best_direct.output_amount > 0.0 {
                    let improvement = ((r.output_amount - best_direct.output_amount)
                        / best_direct.output_amount)
                        * 100.0;
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
