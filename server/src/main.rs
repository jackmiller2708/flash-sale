use flash_sale::app::runtime::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
