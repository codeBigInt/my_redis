use crate::server::create_my_redis_server;

mod command;
mod database;
mod server;

#[tokio::main]
async fn main() {
    let addrs = "127.0.0.1:8440";
    eprintln!("Server will start at {:?}", addrs);

    //Start server
    create_my_redis_server(addrs).await;
}
