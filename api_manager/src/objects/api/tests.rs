#[cfg(test)]
pub mod new {
    use super::super::*;
    #[test]
    fn create_valid_url() {
        // given
        let url = String::from("/api/v1/hoge");
        let method = String::from("GET");
        let parameter = Vec::new();

        // when
        let result = Api::new(url, method, parameter);

        // then
        assert_eq!(result, Ok(Api { url: String::from("/api/v1/hoge"), method: String::from("GET"), parameter: Vec::new() }));
    }

    #[test]
    fn url_heading_non_slash_return_error() {
        // given
        let url = String::from("api/v1/hoge");
        let method = String::from("GET");
        let parameter = Vec::new();

        // when
        let result = Api::new(url, method, parameter);

        // then
        assert_eq!(result, Err(String::from("Invalid URL(url should be start with /)")));
    }

    #[test]
    fn invalid_method_return_error() {
        // given
        let url = String::from("/api/v1/hoge");
        let method = String::from("hoge");
        let parameter = Vec::new();

        // when
        let result = Api::new(url, method, parameter);

        // then
        assert_eq!(result, Err(String::from("Invalid Method(method should be http method[GET,POST,PUT,DELETE])")));
    }

    #[test]
    fn lower_method_return_ok() {
        // given
        let url = String::from("/api/v1/hoge");
        let method = String::from("get");
        let parameter = Vec::new();

        // when
        let result = Api::new(url, method, parameter);

        // then
        assert_eq!(result, Ok(Api { url: String::from("/api/v1/hoge"), method: String::from("GET"), parameter: Vec::new() }));
    }
}
