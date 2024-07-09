use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test_create_user() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    let create_user_request = client.do_post(
        "/api/users",
        json!({
            "username": "sylvainb"
        }),
    );
    create_user_request.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_read_user() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    let read_user_request = client.do_get("/api/users");
    read_user_request.await?.print().await?;

    Ok(())
}
