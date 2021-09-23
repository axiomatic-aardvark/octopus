use crate::MyWorld;
use chrono::{NaiveDateTime, Timelike, Utc};
use cucumber_rust::{t, Steps};
use octopus::api_response::ApiResponse;
use octopus::server_time::ServerTime;

pub fn server_time_steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given(
        "I send a request to fetch the server time",
        |_world, _ctx| MyWorld::Nothing,
    );

    steps.when_async(
        "The server time is returned",
        t!(|_world, _ctx| {
            let time = ApiResponse::<ServerTime>::get("https://api.kraken.com/0/public/Time")
                .await
                .unwrap();

            MyWorld::ServerTime(time)
        }),
    );

    steps.then("It is equal to the current UTC time", |world, _| {
        let now = Utc::now();

        match world {
            MyWorld::ServerTime(time) => assert_eq!(
                NaiveDateTime::from_timestamp(time.unix_time.into(), 0)
                    .time()
                    .minute(),
                NaiveDateTime::from_timestamp(now.timestamp(), 0)
                    .time()
                    .minute()
            ),
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps.given("I send a request with a bad url", |_world, _ctx| {
        MyWorld::Nothing
    });

    steps.when_async(
        "An error is returned",
        t!(|_world, _ctx| {
            match ApiResponse::<ServerTime>::get("https://api.kraken.com/0/public/BadUrl").await {
                Ok(_) => unreachable!(),
                Err(e) => {
                    assert!(e
                        .to_string()
                        .contains("error decoding response body: missing field `result`"));
                }
            };

            MyWorld::Nothing
        }),
    );

    steps
}
