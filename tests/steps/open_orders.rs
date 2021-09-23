use crate::MyWorld;
use cucumber_rust::{t, Steps};
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
            let open_orders = Orders::get().await.unwrap();
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

    steps
}
