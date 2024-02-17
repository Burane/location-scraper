mod afedim;
mod core;
#[allow(warnings, unused)]
mod db;

use afedim::scraper::get_html;
use db::prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;

    let resp = get_html().await?;
    println!("{resp:#?}");
    Ok(())
}

const DATABASE_URL: &str = "sqlite:location_scraper.db";
const DB_NAME: &str = "location_scraper";