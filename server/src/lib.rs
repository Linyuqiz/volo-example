use common::grpc::client::{MICRO_EXAMPLE_CLIENT, MICRO_HELLO_CLIENT};
use tracing::info;
use volo::FastStr;
use volo_gen::{example::GetExampleRequest, hello::GetItemRequest};
use volo_http::server::route::{get, Router};

async fn index_handler() -> &'static str {
    // call helle
    let req = GetItemRequest {
        id: 1,
        name: FastStr::from_static_str("Volo"),
    };
    let resp = MICRO_HELLO_CLIENT.get_item(req).await;
    match resp {
        Ok(info) => {
            info!("Item: {info:?}");
        }
        Err(e) => eprintln!("{e:?}"),
    };

    // call example
    let req = GetExampleRequest {
        id: 1,
        name: FastStr::from_static_str("Volo"),
    };
    let resp = MICRO_EXAMPLE_CLIENT.get_example(req).await;
    match resp {
        Ok(info) => {
            info!("example: {info:?}");
        }
        Err(e) => eprintln!("{e:?}"),
    };

    "It Works!\n"
}

pub fn example_router() -> Router {
    Router::new().route("/", get(index_handler))
}
