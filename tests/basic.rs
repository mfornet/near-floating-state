use near_sdk::serde_json::json;

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = workspaces::compile_project(".").await?;
    let contract = worker.dev_deploy(&wasm).await?;
    let account_id = contract.as_account().id().clone();

    // Initialize the contract
    let init_result = contract.call("new").transact().await?;
    assert!(init_result.is_success());

    // Check get message is empty
    let get_message_result = contract
        .view(
            "get_message",
            near_sdk::serde_json::to_vec(&json!({ "account_id": &account_id }))?,
        )
        .await?;

    let result: Option<String> = get_message_result.json()?;
    assert_eq!(result, None);

    // Add a message
    let add_message_result = contract
        .call("add_message")
        .args_json(json!({ "message": "hello" }))
        .transact()
        .await?;

    assert!(add_message_result.is_success());

    // Check get message again, and verify it is updated
    let get_message_result = contract
        .view(
            "get_message",
            near_sdk::serde_json::to_vec(&json!({ "account_id": &account_id }))?,
        )
        .await?;

    let result: Option<String> = get_message_result.json()?;
    assert_eq!(result, Some("hello".to_string()));

    Ok(())
}
