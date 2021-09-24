use dotenv::dotenv;
use octopus::api_response::ApiResponse;
use octopus::open_orders::Orders;
use octopus::report::Report;
use octopus::server_time::ServerTime;
use octopus::trading_pair::XbtUsd;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let server_time = ApiResponse::<ServerTime>::get("https://api.kraken.com/0/public/Time").await;
    let xbt_usd = ApiResponse::<XbtUsd>::get(
        "https://api.kraken.com/0/public/AssetPairs?pair=XXB\
        TZUSD",
    )
    .await;

    dotenv().ok();
    let api_key = dotenv!("API_KEY");
    let api_secret = dotenv!("API_SECRET");
    let otp = dotenv!("OTP");

    let open_orders = Orders::get(api_key, api_secret, otp).await;

    let report = Report::new(server_time, xbt_usd, open_orders).unwrap_or_else(|e| {
        panic!(
            "An unexpected error occurred while generating the report:\n{}",
            e.to_string()
        )
    });

    println!("{}", report.print_report());
}
