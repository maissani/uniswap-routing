# Uniswap V2 Routing Engine in Rust

🚀 A performant and extensible Uniswap V2 routing engine built in Rust, supporting multiple pathfinding algorithms and slippage-aware optimization.

## 📌 Features

- ✅ Multiple routing algorithms:
  - A\* (AStar)
  - Dijkstra
  - BFS
  - DFS
  - Direct 1-hop route
- ✅ Best route selection logic that **maximizes the received amount** (PnL optimality)
- ✅ Support for multiple hops (configurable via `max_hops`)
- ✅ Extensible graph abstraction for token pairs
- ✅ Designed for speed and correctness (binary heap, deduplication, arc-pool)
- 🔜 Planned: Slippage constraints, gas-aware routing, multi-path aggregation

---

## 🔧 Usage

```rust
let graph = Graph::from_pools(&pools);
let from = Token::from("ETH");
let to = Token::from("USDC");
let amount_in = 10.0;

let algo_auto_selection = RoutingAlgo::Auto;
let slippage = Slippage { tolerance_bps: slippage_bps };

if let Some((algo, r)) = router.compute_route(
side.clone(),
&graph,
&Token(from),
&Token(to),
input,
ExecutionParams { slippage, algo, max_hops: 4 },
) {
if side == Side::Buy {
    print!("→ SIDE BUY:");
} else {
    print!("→ SIDE SELL:");
}
println!("→ Best route algo: {:?}", algo);
if let Some(best_direct) = best_direct(side.clone(), &graph, &Token(from), &Token(to), input, ExecutionParams { algo, slippage, max_hops: 4 }) {
    println!("→ Best direct: {:.6}", best_direct.output_amount);
    println!("→ Best Route Output: {:.6}", r.output_amount);
    if best_direct.output_amount > 0.0 {
        let improvement = ((r.output_amount - best_direct.output_amount) / best_direct.output_amount) * 100.0;
        println!("→ Improvement over best direct swap: {:.2}%", improvement);
    }
    println!("→ No direct swap available");
}

println!("→ Steps: {} hops", r.steps.len());
for step in r.steps.iter() {
    println!("  {} → {} via [{} / {}]", step.from.0, step.to.0, step.pool.token0.0, step.pool.token1.0);
}
} else {
    println!("No route found for {} → {}", from, to);
};

```


## 💡 Routing Logic
All algorithms are run concurrently (A*, Dijkstra, BFS, DFS, and Direct swap), and the route returning the highest output_amount is selected.

You define how much you give (amount_in) and the router finds how much you will get (output_amount) using the best path that maximize value received.

## 📦 Structure
```bash
src/
├── adapter/
│   └── graph.rs        # Graph abstraction for token relationships
├── domain/
│   └── types.rs        # Core domain types (Token, Pool, Route, etc.)
├── application/
│   └── algos/          # All routing algorithm implementations
│       ├── astar.rs
│       ├── bfs.rs
│       ├── dfs.rs
│       ├── dijkstra.rs
│       └── best_direct.rs
│   └── route_engine.rs # Dispatcher that chooses best route
└── main.rs             # Entry point / demo scenarios
```

## ✅ Scenarios Covered
- ✅ ETH → USDC with 10 ETH: find best route to maximize USDC
- ✅ USDC → ETH with 10,000 USDC: find best route to maximize ETH
- ✅ Other scenarios that make the possibility to check the differents ALgos
- ✅ Works for any ERC20 pair with Uniswap V2-compatible pools


## 🚧 TODO
- 📌 Integrate slippage tolerance
- 📌 Perforamance improvement and parralelysm improvement
- 📌 Support for exact-output swaps
- 📌 Gas cost estimation and route pruning
- 📌 real-world compatibility => Transform this demo in a lib that is easy to use