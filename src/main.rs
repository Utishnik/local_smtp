use samotop::io::DummyService;
use samotop::server::TcpServer;
use tokio;
use std::error::Error;
fn main() {
    let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut srv: Result<(), Box<dyn Error + Send + Sync>> =
            TcpServer::on("127.0.0.1:2525").serve(DummyService).await;
    });
}
