#[cfg(test)]
pub mod tests;

#[derive(Debug,PartialEq)]
pub struct Api{
    url:String,
    method:String,
    parameter:Vec<String>
}

impl Api {
    pub fn new(url:String,method:String,parameter:Vec<String>)->Api{
        Api{
            url,
            method,
            parameter
        }
    }
}