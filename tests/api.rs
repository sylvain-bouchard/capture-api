use anyhow::Result;

#[tokio::test]
async fn test_users_endpoint() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    client.do_get("/users").await?;

    Ok(())
}
