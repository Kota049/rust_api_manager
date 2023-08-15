#[cfg(test)]
pub mod tests;

#[derive(Debug,PartialEq)]
pub struct Api{
    url:String,
    method:String,
    parameter:Vec<String>
}

impl Api {
    pub fn new(url:String,method:String,parameter:Vec<String>)->Result<Api,String>{
        if !url.starts_with('/') {
            return Err(String::from("不正なURLです(url should be start with /)"))
        }
        Ok(Api {
            url,
            method,
            parameter,
        })
    }
}