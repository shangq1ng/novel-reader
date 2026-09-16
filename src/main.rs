use novel_reader::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app().await?;

    Ok(())
}
