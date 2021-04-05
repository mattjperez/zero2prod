use zero2prod::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
