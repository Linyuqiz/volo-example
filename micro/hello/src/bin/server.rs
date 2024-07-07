use std::net::SocketAddr;

use common::grpc_adress::MICRO_ADDRESS_HELLO;
use volo::net::Address;
use volo_gen::example::ItemServiceServer;
use volo_grpc::server::{Server, ServiceBuilder};

use hello::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = MICRO_ADDRESS_HELLO.parse().unwrap();
    let addr = Address::from(addr);

    Server::new()
        .add_service(ServiceBuilder::new(ItemServiceServer::new(S)).build())
        .run(addr)
        .await
        .unwrap();
}
