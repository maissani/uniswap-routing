use crate::adapter::graph::Graph;
use crate::application::route_engine;
use crate::domain::types::{ExecutionParams, Route, RoutingAlgo, Side, Token};

pub trait RoutingStrategy {
    fn compute_route(
        &self,
        side: Side,
        graph: &Graph,
        from: &Token,
        to: &Token,
        amount_in: f64,
        params: ExecutionParams,
    ) -> Option<(RoutingAlgo, Route)>;
}

pub struct DefaultRouter;

impl RoutingStrategy for DefaultRouter {
    fn compute_route(
        &self,
        side: Side,
        graph: &Graph,
        from: &Token,
        to: &Token,
        amount_in: f64,
        params: ExecutionParams,
    ) -> Option<(RoutingAlgo, Route)> {
        route_engine::execute(side, graph, from, to, amount_in, params)
    }
}
