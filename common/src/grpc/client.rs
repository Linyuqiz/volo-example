use std::net::SocketAddr;

use lazy_static::lazy_static;

use volo_gen::{
    example::{ExampleServiceClient, ExampleServiceClientBuilder},
    hello::{ItemServiceClient, ItemServiceClientBuilder},
};

use crate::grpc::address::{MICRO_ADDRESS_EXAMPLE, MICRO_ADDRESS_HELLO};

lazy_static! {
    pub static ref MICRO_HELLO_CLIENT: ItemServiceClient = {
        let address: SocketAddr = MICRO_ADDRESS_HELLO.parse().unwrap();
        ItemServiceClientBuilder::new("hello")
            .address(address)
            .build()
    };
}

lazy_static! {
    pub static ref MICRO_EXAMPLE_CLIENT: ExampleServiceClient = {
        let address: SocketAddr = MICRO_ADDRESS_EXAMPLE.parse().unwrap();
        ExampleServiceClientBuilder::new("example")
            .address(address)
            .build()
    };
}
