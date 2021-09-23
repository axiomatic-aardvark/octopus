mod steps;

use cucumber_rust::{async_trait, Cucumber, World};
use octopus::open_orders::Orders;
use octopus::server_time::ServerTime;
use octopus::trading_pair::XbtUsd;
use std::convert::Infallible;

pub enum MyWorld {
    Nothing,
    ServerTime(ServerTime),
    TradingPair(XbtUsd),
    OpenOrders(Orders),
    ApiError(String),
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Nothing)
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<MyWorld>::new()
        .features(&["./features"])
        .steps(steps::server_time::server_time_steps())
        .steps(steps::trading_pair::trading_pair_steps())
        .steps(steps::open_orders::open_orders_steps())
        .run_and_exit()
        .await
}
