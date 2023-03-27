use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use hello_world::calculator_service_server::{CalculatorService, CalculatorServiceServer};
use hello_world::{SumRequest, SumResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
    }
}

pub struct MyCalculator {}

#[tonic::async_trait]
impl CalculatorService for MyCalculator {
    fn sum(
        &self,
        request: tonic::Request<hello_world::SumRequest>,
    ) -> Result<Response<SumResponse>, Status> {
        let sum = request.into_inner().a + request.into_inner().b;

        let response: SumResponse = SumResponse { sum: sum };

        Ok(Response::new(response));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
