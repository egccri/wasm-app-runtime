use invoker_grpc::server::caller::caller_client::CallerClient;
use invoker_grpc::server::caller::CallRequest;
use std::error::Error;

type BoxError = Box<dyn Error>;

// Run egccri runtime and install service 1
#[tokio::test]
async fn invoke() -> Result<(), BoxError> {
    let mut client = CallerClient::connect("").await?;
    let request = tonic::Request::new(CallRequest {
        service_id: 1,
        payload: "payload".to_string(),
    });
    let response = client.call(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
