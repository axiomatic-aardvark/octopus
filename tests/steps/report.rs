use crate::MyWorld;
use cucumber_rust::{t, Steps};
use dotenv::dotenv;
use octopus::api_response::ApiResponse;
use octopus::open_orders::Orders;
use octopus::report::Report;
use octopus::server_time::ServerTime;

pub fn report_steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given_async(
        "All three components are fetched successfully",
        t!(|_world, _ctx| {
            dotenv().ok();
            let api_key = dotenv!("API_KEY");
            let api_secret = dotenv!("API_SECRET");
            let otp = dotenv!("OTP");

            let time = ApiResponse::<ServerTime>::get("https://api.kraken.com/0/public/Time").await;

            let xbt_usd =
                ApiResponse::get("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD").await;

            let open_orders = Orders::get(api_key, api_secret, otp).await;

            MyWorld::ReportCtx((time, xbt_usd, open_orders))
        }),
    );

    steps.when(
        "I generate a report without errors",
        |world, _ctx| match world {
            MyWorld::ReportCtx((time, pair, orders)) => {
                let report = Report::new(time, pair, orders);
                MyWorld::Report(report.unwrap())
            }
            _ => panic!("Invalid world state"),
        },
    );

    steps.then(
        "The report without errors is correctly generated",
        |world, _| {
            match world {
                MyWorld::Report(r) => {
                    assert!(r.print_report().contains("Server time"));
                    assert!(r.print_report().contains("XBT-USD trading pair"));
                    assert!(r.print_report().contains("Orders"));

                    assert!(r.print_report().contains("unixtime"));
                    assert!(r.print_report().contains("XBTUSD"));
                    assert!(r.print_report().contains("Orders"));
                }
                _ => panic!("Invalid world state"),
            }
            MyWorld::Nothing
        },
    );

    steps.given_async(
        "All three components are fetched successfully",
        t!(|_world, _ctx| {
            dotenv().ok();
            let api_key = dotenv!("API_KEY");
            let api_secret = dotenv!("API_SECRET");
            let otp = dotenv!("OTP");

            let time = ApiResponse::<ServerTime>::get("https://api.kraken.com/0/public/Time").await;

            let xbt_usd =
                ApiResponse::get("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD").await;

            let open_orders = Orders::get(api_key, api_secret, otp).await;

            MyWorld::ReportCtx((time, xbt_usd, open_orders))
        }),
    );

    steps.when(
        "I generate a report without errors",
        |world, _ctx| match world {
            MyWorld::ReportCtx((time, pair, orders)) => {
                let report = Report::new(time, pair, orders);
                MyWorld::Report(report.unwrap())
            }
            _ => panic!("Invalid world state"),
        },
    );

    steps.then(
        "The report without errors is correctly generated",
        |world, _| {
            match world {
                MyWorld::Report(r) => {
                    assert!(r.print_report().contains("Server time"));
                    assert!(r.print_report().contains("XBT-USD trading pair"));
                    assert!(r.print_report().contains("Orders"));

                    assert!(r.print_report().contains("unixtime"));
                    assert!(r.print_report().contains("XBTUSD"));
                    assert!(r.print_report().contains("Orders"));
                }
                _ => panic!("Invalid world state"),
            }
            MyWorld::Nothing
        },
    );

    steps.given_async(
        "One component fails to be fetched",
        t!(|_world, _ctx| {
            dotenv().ok();
            let api_key = dotenv!("API_KEY");
            let api_secret = dotenv!("API_SECRET");
            let otp = dotenv!("OTP");

            let time = ApiResponse::<ServerTime>::get("https://api.kraken.com/0/public/Time")
                .await;

            let xbt_usd =
                ApiResponse::get("https://api.kraken.com/0/public/AssetPairs?pair=INVALID_PAIR")
                    .await;


            let open_orders = Orders::get(api_key, api_secret, otp).await;

            MyWorld::ReportCtx((time, xbt_usd, open_orders))
        }),
    );

    steps.when(
        "I generate a report with one error and two successful reports",
        |world, _ctx| match world {
            MyWorld::ReportCtx((time, pair, orders)) => {
                let report = Report::new(time, pair, orders);
                MyWorld::Report(report.unwrap())
            }
            _ => panic!("Invalid world state"),
        },
    );

    steps.then(
        "The report with with one error and two successful reports is correctly generated",
        |world, _| {
            match world {
                MyWorld::Report(r) => {
                    assert!(r.print_report().contains("Server time"));
                    assert!(r.print_report().contains("XBT-USD trading pair"));
                    assert!(r.print_report().contains("Orders"));

                    assert!(r.print_report().contains("unixtime"));
                    assert!(r.print_report().contains("[\"EQuery:Unknown asset pair\"]"));
                    assert!(r.print_report().contains("Orders"));
                }
                _ => panic!("Invalid world state"),
            }
            MyWorld::Nothing
        },
    );

    steps.given_async(
        "All components fail to be fetched",
        t!(|_world, _ctx| {
            dotenv().ok();
            let api_key = dotenv!("API_KEY");
            let api_secret = dotenv!("API_SECRET");
            let otp = "INVALID_OTP";

            let time = ApiResponse::<ServerTime>::get("https://api.kraken.com/0/public/INVALID_ENDPOINT")
                .await;

            let xbt_usd =
                ApiResponse::get("https://api.kraken.com/0/public/AssetPairs?pair=INVALID_PAIR")
                    .await;


            let open_orders = Orders::get(api_key, api_secret, otp).await;

            MyWorld::ReportCtx((time, xbt_usd, open_orders))
        }),
    );

    steps.when(
        "I generate a report with three error messages",
        |world, _ctx| match world {
            MyWorld::ReportCtx((time, pair, orders)) => {
                let report = Report::new(time, pair, orders);
                MyWorld::Report(report.unwrap())
            }
            _ => panic!("Invalid world state"),
        },
    );

    steps.then(
        "The report with three error messages is correctly generated",
        |world, _| {
            match world {
                MyWorld::Report(r) => {
                    assert!(r.print_report().contains("Server time"));
                    assert!(r.print_report().contains("XBT-USD trading pair"));
                    assert!(r.print_report().contains("Orders"));

                    assert!(r.print_report().contains("[\"EGeneral:Unknown method\"]"));
                    assert!(r.print_report().contains("[\"EQuery:Unknown asset pair\"]"));
                    assert!(r.print_report().contains("[\"EAPI:Invalid signature\"]"));
                }
                _ => panic!("Invalid world state"),
            }
            MyWorld::Nothing
        },
    );

    steps
}
