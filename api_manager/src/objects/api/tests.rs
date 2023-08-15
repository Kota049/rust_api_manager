use super::*;

#[test]
fn create_valid_url(){
    // given
    let url = String::from("/api/v1/hoge");
    let method = String::from("GET");
    let parameter = Vec::new();

    // when
    let result = Api::new(url,method,parameter);

    // then
    assert_eq!(result, Ok(Api { url: String::from("/api/v1/hoge"), method: String::from("GET"), parameter: Vec::new() }));
}

#[test]
fn url_heading_non_slash_return_error(){
    // given
    let url = String::from("api/v1/hoge");
    let method = String::from("GET");
    let parameter = Vec::new();

    // when
    let result = Api::new(url,method,parameter);

    // then
    assert_eq!(result, Err(String::from("不正なURLです(url should be start with /)")));
}