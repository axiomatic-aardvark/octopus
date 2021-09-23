use chrono::{NaiveDateTime, Timelike, Utc};
use cucumber_rust::{async_trait, t, Cucumber, Steps, World};
use octopus::api_response::ApiResponse;
use octopus::server_time::ServerTime;
use octopus::trading_pair::XbtUsd;
use std::convert::Infallible;
use octopus::open_orders::Orders;

pub enum MyWorld {
    Nothing,
    ServerTime(ServerTime),
    TradingPair(XbtUsd),
    OpenOrders(Orders)
}

// TODO: Extract steps into separate files to reduce bloat
pub fn server_time_steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given(
        "I send a request to fetch the server time",
        |_world, _ctx| MyWorld::Nothing,
    );

    steps.when_async(
        "The server time is returned",
        t!(|_world, _ctx| {
            let time = ApiResponse::<ServerTime>::get(String::from(
                "https://api.kraken.com/0/public/Time",
            ))
            .await
            .unwrap();

            MyWorld::ServerTime(time)
        }),
    );

    steps.then("It is equal to the current UTC time", |world, _| {
        let now = Utc::now();

        match world {
            MyWorld::ServerTime(time) => assert_eq!(
                NaiveDateTime::from_timestamp(time.unix_time.into(), 0).minute(),
                NaiveDateTime::from_timestamp(now.timestamp(), 0).minute()
            ),
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps
}

pub fn trading_pair_steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given(
        "I send a request to fetch the trading pair info for XBT-USD",
        |_world, _ctx| MyWorld::Nothing,
    );

    steps.when_async(
        "The trading pair info is returned",
        t!(|_world, _ctx| {
            let xbt_usd = ApiResponse::<XbtUsd>::get(String::from(
                "https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD",
            ))
            .await
            .unwrap();

            MyWorld::TradingPair(xbt_usd)
        }),
    );

    steps.then("The trading pair response is valid", |world, _| {
        match world {
            MyWorld::TradingPair(xbt_usd) => {
                assert_eq!(xbt_usd.pair_info.alt_name, "XBTUSD");
            },
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps
}

pub fn open_orders_steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given(
        "I send a request to fetch the open orders for an account",
        |_world, _ctx| MyWorld::Nothing,
    );

    steps.when_async(
        "The open orders are returned",
        t!(|_world, _ctx| {
            let open_orders = Orders::get().await.unwrap();
           MyWorld::OpenOrders(open_orders)
        }),
    );

    steps.then("The open orders response is valid", |world, _| {
        match world {
            MyWorld::OpenOrders(_o) => {},
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps
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
        .steps(server_time_steps())
        .steps(trading_pair_steps())
        .steps(open_orders_steps())
        .run_and_exit()
        .await
}
