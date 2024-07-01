use anyhow::Result;

#[tokio::test]
async fn test_endpoint() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client.do_get("/users").await?.print.await?;

    Ok()
}
