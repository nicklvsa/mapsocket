mod types;
mod server;
mod handlers;

fn main() {
    println!("Starting server...");
    
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            server::serve().await;
        })
}
