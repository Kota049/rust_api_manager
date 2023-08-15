use std::fs;
use serde_json::json;
use super::super::constants::FILE_NAME;
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
    store(&value).await.unwrap();

    // then
    let content = fs::read_to_string(FILE_NAME).unwrap();
    assert_eq!(content, serde_json::to_string_pretty(&value).unwrap() + "\n");
}

#[tokio::test]
async fn store_json_string_include_multi_api() {
    // given
    let value = json!([
        {
        "url":"/api/v1/hoge",
        "method":"GET",
        "parameter":["email"],
        },
        {
        "url":"/api/v1/hoge2",
        "method":"GET",
        "parameter":["email2"],
        },
    ]);

    // when
    store(&value).await.unwrap();

    // then
    let content = fs::read_to_string(FILE_NAME).unwrap();
    assert_eq!(content, serde_json::to_string_pretty(&value).unwrap() + "\n");
}