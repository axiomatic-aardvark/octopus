use crate::common::assert_api_error;
use crate::MyWorld;
use cucumber_rust::{t, Steps};
use dotenv::dotenv;
use octopus::open_orders::Orders;

pub fn open_orders_steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given(
        "I send a request to fetch the open orders for an account",
        |_world, _ctx| MyWorld::Nothing,
    );

    steps.when_async(
        "The open orders are returned",
        t!(|_world, _ctx| {
            dotenv().ok();
            let api_key = dotenv!("API_KEY");
            let api_secret = dotenv!("API_SECRET");
            let otp = dotenv!("OTP");

            let open_orders = Orders::get(api_key, api_secret, otp).await.unwrap();
            MyWorld::OpenOrders(open_orders)
        }),
    );

    steps.then("The open orders response is valid", |world, _| {
        match world {
            MyWorld::OpenOrders(_o) => {}
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps.given(
        "I send a request to fetch the open orders for an account with invalid otp",
        |_world, _ctx| MyWorld::Nothing,
    );

    steps.when_async(
        "A invalid signature error is returned",
        t!(|_world, _ctx| {
            dotenv().ok();
            let api_key = dotenv!("API_KEY");
            let api_secret = dotenv!("API_SECRET");
            let otp = "INVALID_OTP";

            match Orders::get(api_key, api_secret, otp).await {
                Ok(_) => unreachable!(),
                Err(e) => MyWorld::ApiError(e.to_string()),
            }
        }),
    );

    steps.then(
        "The invalid signature error is reported properly",
        |world, _| assert_api_error(world, "[\"EAPI:Invalid signature\"]"),
    );

    steps
}
