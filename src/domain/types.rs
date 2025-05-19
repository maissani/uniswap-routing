use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(pub &'static str);

#[derive(Debug, Clone)]
pub struct Pool {
    pub token0: Token,
    pub token1: Token,
    pub reserve0: f64,
    pub reserve1: f64,
    pub fee_bps: u32,
}

impl Pool {
    pub fn supports(&self, a: &Token, b: &Token) -> bool {
        (a == &self.token0 && b == &self.token1) || (a == &self.token1 && b == &self.token0)
    }

    pub fn get_output_amount(&self, input_token: &Token, input_amount: f64) -> Option<f64> {
        let fee_multiplier = 1.0 - (self.fee_bps as f64 / 10_000.0);
        let input_amount_with_fee = input_amount * fee_multiplier;

        let (reserve_in, reserve_out) = if input_token == &self.token0 {
            (self.reserve0, self.reserve1)
        } else if input_token == &self.token1 {
            (self.reserve1, self.reserve0)
        } else {
            return None;
        };

        let output = (input_amount_with_fee * reserve_out) / (reserve_in + input_amount_with_fee);
        Some(output)
    }

    pub fn get_other_token(&self, token: &Token) -> Option<Token> {
        if token == &self.token0 {
            Some(self.token1.clone())
        } else if token == &self.token1 {
            Some(self.token0.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwapStep {
    pub from: Token,
    pub to: Token,
    pub pool: Arc<Pool>,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub steps: Vec<SwapStep>,
    pub output_amount: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct FeeParams {
    pub fee_bps: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Slippage {
    pub tolerance_bps: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoutingAlgo {
    BestDirect,
    Bfs,
    Dfs,
    Dijkstra,
    AStar,
    Auto,
}

#[derive(Debug, Clone, Copy)]
pub struct ExecutionParams {
    pub algo: RoutingAlgo,
    pub slippage: Slippage,
    pub max_hops: usize,
}
