use std::net::SocketAddr;

use common::grpc::address::MICRO_ADDRESS_EXAMPLE;
use volo::net::Address;
use volo_gen::example::ExampleServiceServer;
use volo_grpc::server::{Server, ServiceBuilder};

use example::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = MICRO_ADDRESS_EXAMPLE.parse().unwrap();
    let addr = Address::from(addr);

    Server::new()
        .add_service(ServiceBuilder::new(ExampleServiceServer::new(S)).build())
        .run(addr)
        .await
        .unwrap();
}
