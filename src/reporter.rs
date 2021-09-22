use anyhow::Result;

use crate::open_orders::Orders;
use crate::server_time::ServerTime;
use crate::trading_pair::XbtUsd;

pub struct Reporter {
    server_time_report: String,
    xbt_usd_report: String,
    orders_report: String,
}

impl Reporter {
    pub fn new(
        server_time: Result<ServerTime>,
        xbt_usd: Result<XbtUsd>,
        orders: Result<Orders>,
    ) -> Result<Self> {
        let server_time_report = match server_time {
            Ok(t) => serde_json::to_string_pretty(&t)?,
            Err(e) => e.to_string(),
        };
        let xbt_usd_report = match xbt_usd {
            Ok(p) => serde_json::to_string_pretty(&p)?,
            Err(e) => e.to_string(),
        };
        let orders_report = match orders {
            Ok(o) => serde_json::to_string_pretty(&o)?,
            Err(e) => e.to_string(),
        };

        Ok(Reporter {
            server_time_report,
            xbt_usd_report,
            orders_report,
        })
    }

    pub fn print_report(&self) -> String {
        format!(
            "Server time:\n{}\nXBT-USD trading pair:\n{}\nOpen Orders:\n{}",
            self.server_time_report, self.xbt_usd_report, self.orders_report
        )
    }
}
