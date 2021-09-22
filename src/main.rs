use octopus::server_time::ServerTime;
use octopus::api_response::ApiResponse;
use octopus::trading_pair::XbtUsd;
use octopus::open_orders::Orders;
use octopus::reporter::Reporter;

#[tokio::main]
async fn main () {
    // TODO: Try to make these str slices
    let server_time = ApiResponse::<ServerTime>::get(String::from("https://api.kraken.com/0/public/Time")).await;
    let xbt_usd = ApiResponse::<XbtUsd>::get(String::from("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD")).await;
    let open_orders = Orders::get().await;

    let reporter = Reporter::new(server_time, xbt_usd, open_orders).unwrap_or_else(|e| {
        panic!("An unexpected error occurred while generating the report:\n{}", e.to_string())
    });

    println!("{}", reporter.print_report());
}