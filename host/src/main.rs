mod api;

use api::api_start;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    // load ENV variables
    dotenv().ok();

    // start API
    api_start().await;
}