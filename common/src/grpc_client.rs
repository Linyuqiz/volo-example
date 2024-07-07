use std::net::SocketAddr;

use lazy_static::lazy_static;

use crate::grpc_adress::{MICRO_ADDRESS_EXAMPLE, MICRO_ADDRESS_HELLO};

use volo_gen::example::{
    ExampleServiceClient, ExampleServiceClientBuilder, ItemServiceClient, ItemServiceClientBuilder,
};

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
