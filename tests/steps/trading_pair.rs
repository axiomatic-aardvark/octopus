use crate::MyWorld;
use cucumber_rust::{t, Steps};
use octopus::api_response::ApiResponse;
use octopus::trading_pair::XbtUsd;

pub fn trading_pair_steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given(
        "I send a request to fetch the trading pair info for XBT-USD",
        |_world, _ctx| MyWorld::Nothing,
    );

    steps.when_async(
        "The trading pair info is returned",
        t!(|_world, _ctx| {
            let xbt_usd = ApiResponse::<XbtUsd>::get(
                "https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD",
            )
            .await
            .unwrap();

            MyWorld::TradingPair(xbt_usd)
        }),
    );

    steps.then("The trading pair response is valid", |world, _| {
        match world {
            MyWorld::TradingPair(xbt_usd) => {
                assert_eq!(xbt_usd.pair_info.alt_name, "XBTUSD");
            }
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps
}
