use std::fs;
use serde_json::json;
use super::*;

#[tokio::test]
async fn store_json_string() {
    // given
    let value = json!({
        "url":"/api/v1/hoge",
        "method":"GET",
        "parameter":["email"],
    });

    // when
    store(value.clone()).await;

    // then
    let content = fs::read_to_string("api_manager.json").unwrap();
    assert_eq!(content, value.to_string());
}