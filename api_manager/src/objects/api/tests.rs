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
    assert_eq!(result, Api{url:String::from("/api/v1/hoge"),method:String::from("GET"),parameter:Vec::new()});
}