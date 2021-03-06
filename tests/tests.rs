mod common;
mod steps;
use anyhow::Result;
use cucumber_rust::{async_trait, Cucumber, World};
use octopus::open_orders::Orders;
use octopus::report::Report;
use octopus::server_time::ServerTime;
use octopus::trading_pair::XbtUsd;
use std::convert::Infallible;

#[macro_use]
extern crate dotenv_codegen;

pub enum MyWorld {
    Nothing,
    ServerTime(ServerTime),
    TradingPair(XbtUsd),
    OpenOrders(Orders),
    ApiError(String),
    ReportCtx((Result<ServerTime>, Result<XbtUsd>, Result<Orders>)),
    Report(Report),
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
        .steps(steps::report::report_steps())
        .run_and_exit()
        .await
}
