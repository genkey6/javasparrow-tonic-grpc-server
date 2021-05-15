use tonic::{transport::Server, Request, Response, Status};

use javasparrow::javasparrow_api_server::{JavasparrowApi, JavasparrowApiServer};
use javasparrow::{PiyoRequest, PiyoResponse};

pub mod javasparrow {
    // proto で定義した package 名を指定すると、自動生成した
    // server, client のコードをインポートしてくれる
    tonic::include_proto!("javasparrow");
}

#[derive(Debug, Default)]
pub struct MyJavasparrowApi {}

#[tonic::async_trait]
impl JavasparrowApi for MyJavasparrowApi {
    async fn piyo(&self, request: Request<PiyoRequest>) -> Result<Response<PiyoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = javasparrow::PiyoResponse {
            message: format!("PiyoPiyo {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let javasparrow = MyJavasparrowApi::default();

    Server::builder()
        .add_service(JavasparrowApiServer::new(javasparrow))
        .serve(addr)
        .await?;

    Ok(())
}
