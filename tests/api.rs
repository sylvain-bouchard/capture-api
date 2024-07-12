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

    let read_user_request = client.do_get("/api/users/sylvainb");
    read_user_request.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_list_users() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    let list_users_request = client.do_get("/api/users");
    list_users_request.await?.print().await?;

    Ok(())
}
