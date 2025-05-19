use crate::domain::types::{Pool, Token};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug)]
pub struct Graph {
    pub adjacency: HashMap<Token, Vec<Arc<Pool>>>,
}

/// Represents a graph structure where tokens are connected via pools.
///
/// # Fields
///
/// - `adjacency`: A `HashMap` where the keys are tokens and the values are vectors of pools
///   that connect to the respective token.
///
/// # Methods
///
/// - `new`: Constructs a new `Graph` instance from a slice of pools. It builds the adjacency
///   list by associating each token in the pools with the corresponding pool.
///
/// - `neighbors`: Returns an iterator over the neighbors of a given token. Each neighbor is
///   represented as a tuple containing the other token in the pool and a reference to the pool.
///
/// - `tokens`: Returns a `HashSet` containing all the tokens present in the graph.
impl Graph {
    pub fn new(pools: &[Arc<Pool>]) -> Self {
        let mut adjacency: HashMap<Token, Vec<Arc<Pool>>> = HashMap::new();

        for pool in pools {
            adjacency
                .entry(pool.token0.clone())
                .or_default()
                .push(Arc::clone(pool));
            adjacency
                .entry(pool.token1.clone())
                .or_default()
                .push(Arc::clone(pool));
        }

        Self { adjacency }
    }

    /// Constructs a new `Graph` instance from a slice of pools.
    ///
    /// # Arguments
    ///
    /// * `pools`: A slice of `Arc<Pool>` representing the pools to be added to the graph.
    ///     
    /// # Returns
    ///
    /// A new `Graph` instance with the adjacency list built from the provided pools.
    pub fn neighbors(&self, token: &Token) -> impl ParallelIterator<Item = (Token, Arc<Pool>)> {
        self.adjacency
            .get(token)
            .into_par_iter()
            .flatten()
            .filter_map(move |pool| {
                pool.get_other_token(token)
                    .map(|other| (other, Arc::clone(pool)))
            })
    }

    /// Returns a `HashSet` containing all the tokens present in the graph.
    pub fn tokens(&self) -> HashSet<&Token> {
        self.adjacency.keys().collect()
    }
}
