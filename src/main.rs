use samotop::io::DummyService;
use samotop::server::TcpServer;
fn main() {
    println!("Server start");
    let srv =
        TcpServer::on("127.0.0.1:2525").serve(DummyService);
    async_std::task::block_on(srv).unwrap();
    println!("Server close");
}
