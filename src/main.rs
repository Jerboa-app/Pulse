use pulse::server::serve;

#[tokio::main]
async fn main() {

    serve().await;
    
}