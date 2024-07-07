use std::result::Result;
use volo_gen::example::{GetItemRequest, GetItemResponse, ItemService};
use volo_grpc::{Request, Response, Status};

pub struct S;

impl ItemService for S {
    async fn get_item(
        &self,
        _req: Request<GetItemRequest>,
    ) -> Result<Response<GetItemResponse>, Status> {
        Ok(Response::new(Default::default()))
    }
}
