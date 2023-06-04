use tonic::transport::Server;

pub async fn start(addr: &str) {
    let addr = addr.parse().unwrap();
    Server::builder().add_service(
        todo!()
    ).serve(addr).await?
}