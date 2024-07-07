use std::result::Result;
use volo_gen::example::{GetExampleRequest, GetExampleResponse, ExampleService};
use volo_grpc::{Request, Response, Status};

pub struct S;

impl ExampleService for S {
    async fn get_example(
        &self,
        _req: Request<GetExampleRequest>,
    ) -> Result<Response<GetExampleResponse>, Status> {
        Ok(Response::new(Default::default()))
    }
}
